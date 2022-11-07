use eframe::{
    egui::{self, Rect, Ui},
    epaint::{Pos2, Vec2},
};

use owlchess::{Board, Color, File, Make, Piece, Rank};

use self::{pieces_images::PiecesImages, utils::get_uci_move_for};

mod painter;
mod pieces_images;
mod utils;

#[derive(Debug)]
pub(crate) struct DndData {
    piece_type: Piece,
    piece_color: Color,
    x: f32,
    y: f32,
    start_file: u8,
    start_rank: u8,
    end_file: u8,
    end_rank: u8,
    has_pending_promotion: bool,
    started_with_reversed_board: bool,
}

pub struct ChessBoard {
    size: f32,
    pieces_images: PiecesImages,
    position: Board,
    reversed: bool,
    dnd_data: Option<DndData>,
    on_move_done: Box<dyn Fn(&String) -> ()>,
}

impl ChessBoard {
    pub fn new(size: f32, on_move_done: Box<dyn Fn(&String) -> ()>) -> Self {
        Self {
            size,
            pieces_images: PiecesImages::new(),
            position: Board::initial(),
            reversed: false,
            dnd_data: None,
            on_move_done,
        }
    }

    pub fn widget(&mut self) -> impl egui::Widget + '_ {
        move |ui: &mut egui::Ui| self.view(ui)
    }

    pub fn toggle_orientation(&mut self) {
        self.reversed = !self.reversed;
    }

    fn view(&mut self, ui: &mut Ui) -> egui::Response {
        // 1. Deciding widget size:
        let desired_size = egui::vec2(self.size, self.size);

        // 2. Allocating space:
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::drag());

        // 3. Interact: Time to check for clicks!
        if response.drag_started() {
            let location = response.ctx.pointer_interact_pos().unwrap();
            let location = location - Pos2::ZERO;
            self.handle_drag_started(location, rect);
        } else if response.drag_released() {
            let location = response.ctx.pointer_interact_pos().unwrap();
            let location = location - Pos2::ZERO;
            self.handle_drag_released(location, rect);
        } else if response.dragged() {
            let location = response.ctx.pointer_interact_pos().unwrap();
            let location = location - Pos2::ZERO;
            self.handle_drag(location, rect);
        }

        // 4. Paint!
        // Make sure we need to paint:
        if ui.is_rect_visible(rect) {
            painter::draw_background(ui, rect);
            painter::draw_cells(ui, rect, &self);
            painter::draw_pieces(ui, rect, &self);
            painter::draw_coordinates(ui, rect, self);
            painter::draw_player_turn(ui, rect, &self);
            painter::draw_moved_piece(ui, rect, &self);
            painter::draw_promotion_buttons(ui, rect, self);
        }
        response
    }

    fn handle_drag_started(&mut self, location: Vec2, rect: Rect) {
        if let Some(dnd_data) = &self.dnd_data {
            if dnd_data.has_pending_promotion {
                return;
            }
        }

        let size = rect.size().x;
        let cells_size = size * 0.111;

        let x = location.x - rect.min.x;
        let y = location.y - rect.min.y;

        let col = ((x - cells_size * 0.5) / cells_size).floor() as i32;
        let row = ((y - cells_size * 0.5) / cells_size).floor() as i32;

        if col < 0 || col > 7 || row < 0 || row > 7 {
            return;
        }

        let col = col as u8;
        let row = row as u8;

        let file = if self.reversed { 7 - col } else { col };
        let rank = if self.reversed { row } else { 7 - row };

        let square = self.position.get2(
            File::from_index(file as usize),
            Rank::from_index(7 - rank as usize),
        );
        if square.is_free() {
            return;
        }

        let piece_type = square.piece().unwrap();
        let piece_color = square.color().unwrap();

        let player_turn = self.position.side();
        let is_not_our_turn = player_turn != piece_color;

        if is_not_our_turn {
            return;
        }

        self.dnd_data = Some(DndData {
            x,
            y,
            piece_type,
            piece_color,
            start_file: file,
            start_rank: rank,
            end_file: file,
            end_rank: rank,
            has_pending_promotion: false,
            started_with_reversed_board: self.reversed,
        });
    }

    fn handle_drag_released(&mut self, location: Vec2, rect: Rect) {
        if self.dnd_data.is_none() {
            return;
        }

        let dnd_data = self.dnd_data.as_mut().unwrap();
        if dnd_data.has_pending_promotion {
            return;
        }

        let size = rect.size().x;
        let cells_size = size * 0.111;

        let x = location.x - rect.min.x;
        let y = location.y - rect.min.y;

        let col = ((x - cells_size * 0.5) / cells_size).floor() as i32;
        let row = ((y - cells_size * 0.5) / cells_size).floor() as i32;

        if col < 0 || col > 7 || row < 0 || row > 7 {
            self.dnd_data = None;
            return;
        }

        let col = col as u8;
        let row = row as u8;

        let file = if self.reversed { 7 - col } else { col };
        let rank = if self.reversed { row } else { 7 - row };

        let dnd_data = self.dnd_data.as_mut().unwrap();

        let start_square = self.position.get2(
            File::from_index(dnd_data.start_file as usize),
            Rank::from_index(7 - dnd_data.start_rank as usize),
        );
        let is_promotion = match start_square.piece() {
            Some(piece_type) => match start_square.color() {
                Some(piece_color) => {
                    piece_type == Piece::Pawn
                        && ((piece_color == Color::White && rank == 7)
                            || (piece_color == Color::Black && rank == 0))
                }
                _ => false,
            },
            _ => false,
        };

        if is_promotion {
            dnd_data.end_file = file;
            dnd_data.end_rank = rank;
            dnd_data.has_pending_promotion = true;
            return;
        }

        if let Some(dnd_data) = &self.dnd_data {
            let uci_move = get_uci_move_for(
                dnd_data.start_file,
                dnd_data.start_rank,
                file as u8,
                rank as u8,
                None,
            );
            let matching_move = uci_move.into_move(&self.position);

            let move_san = match matching_move {
                Ok(matching_move) => match matching_move.san(&self.position) {
                    Ok(san) => Some(san.to_string()),
                    _ => None,
                },
                Err(_) => None,
            };

            if let Ok(matching_move) = matching_move {
                match matching_move.make_raw(&mut self.position) {
                    Ok(_) => {
                        let white_turn_before_move = self.position.side() == Color::Black;
                        (self.on_move_done)(&utils::san_to_fan(move_san.unwrap(), white_turn_before_move));
                    },
                    _ => {}
                }
            }
        }

        self.dnd_data = None;
    }

    fn handle_drag(&mut self, location: Vec2, rect: Rect) {
        match &mut self.dnd_data {
            Some(dnd_data) => {
                if dnd_data.has_pending_promotion {
                    return;
                }
                let size = rect.size().x;
                let cells_size = size * 0.111;

                let x = location.x - rect.min.x;
                let y = location.y - rect.min.y;

                let col = ((x - cells_size * 0.5) / cells_size).floor() as i32;
                let row = ((y - cells_size * 0.5) / cells_size).floor() as i32;

                let col = col as u8;
                let row = row as u8;

                let file = if self.reversed { 7 - col } else { col };
                let rank = if self.reversed { 7 - row } else { row };

                dnd_data.x = location.x;
                dnd_data.y = location.y;
                dnd_data.end_file = file;
                dnd_data.end_rank = rank;
            }
            _ => {}
        }
    }

    fn commit_promotion(&mut self, promotion_type: char) {
        // There must be a pending promotion
        match &self.dnd_data {
            Some(dnd_data) => {
                if !dnd_data.has_pending_promotion {
                    return;
                }
            }
            _ => return,
        }

        let dnd_data = self.dnd_data.as_ref().unwrap();

        let uci_move = get_uci_move_for(
            dnd_data.start_file,
            dnd_data.start_rank,
            dnd_data.end_file,
            dnd_data.end_rank,
            Some(promotion_type.to_ascii_lowercase()),
        );

        let matching_move = uci_move.into_move(&self.position);

        let move_san = match matching_move {
            Ok(matching_move) => match matching_move.san(&self.position) {
                Ok(san) => Some(san.to_string()),
                _ => None,
            },
            Err(_) => None,
        };

        if let Ok(matching_move) = matching_move {
            match matching_move.make_raw(&mut self.position) {
                Ok(_) => {
                    let white_turn_before_move = self.position.side() == Color::Black;
                    (self.on_move_done)(&utils::san_to_fan(move_san.unwrap(), white_turn_before_move));
                },
                _ => {}
            }
        }

        self.dnd_data = None;
    }
}
