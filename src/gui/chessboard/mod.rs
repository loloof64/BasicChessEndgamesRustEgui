use eframe::{
    egui::{self, Rect, Ui},
    epaint::{Pos2, Vec2},
};

use owlchess::{Board, Color, File, Piece, Rank};

use self::pieces_images::PiecesImages;

mod painter;
mod pieces_images;
mod utils;

pub(crate) struct DndData {
    piece_type: Piece,
    piece_color: Color,
    x: f32,
    y: f32,
    start_file: u8,
    start_rank: u8,
}

pub struct ChessBoard {
    size: f32,
    pieces_images: PiecesImages,
    position: Board,
    reversed: bool,
    dnd_data: Option<DndData>,
}

impl ChessBoard {
    pub fn new(size: f32) -> Self {
        Self {
            size,
            pieces_images: PiecesImages::new(),
            position: Board::from_fen(
                "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
            )
            .unwrap(),
            reversed: false,
            dnd_data: None,
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
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

        // 3. Interact: Time to check for clicks!
        if response.clicked() {
            ////////////////////////////
            println!("Clicked !");
            ////////////////////////////
        } else if response.drag_started() {
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
            painter::draw_cells(ui, rect);
            painter::draw_pieces(
                ui,
                rect,
                &self.pieces_images,
                self.position.clone(),
                self.reversed,
                &self.dnd_data,
            );
            painter::draw_coordinates(ui, rect, self.reversed);
            painter::draw_player_turn(ui, rect, self.position.clone());
            painter::draw_moved_piece(ui, rect, &self.dnd_data, &self.pieces_images);
        }
        response
    }

    fn handle_drag_started(&mut self, location: Vec2, rect: Rect) {
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
        let rank = if self.reversed { 7 - row } else { row };

        let square = self.position.get2(
            File::from_index(file as usize),
            Rank::from_index(rank as usize),
        );
        if square.is_free() {
            return;
        }

        let piece_type = square.piece().unwrap();
        let piece_color = square.color().unwrap();

        self.dnd_data = Some(DndData {
            x,
            y,
            piece_type,
            piece_color,
            start_file: file,
            start_rank: rank,
        });
    }

    fn handle_drag_released(&mut self, location: Vec2, rect: Rect) {
        self.dnd_data = None;
    }

    fn handle_drag(&mut self, location: Vec2, rect: Rect) {
        match &mut self.dnd_data {
            Some(dnd_data) => {
                dnd_data.x = location.x;
                dnd_data.y = location.y;
            }
            _ => {}
        }
    }
}
