use eframe::egui::{self, Ui};

use owlchess::Board;

use self::pieces_images::PiecesImages;

mod painter;
mod pieces_images;
mod utils;

pub struct ChessBoard {
    size: f32,
    pieces_images: PiecesImages,
    position: Board,
    reversed: bool,
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
        }
    }

    pub fn widget(&self) -> impl egui::Widget + '_ {
        move |ui: &mut egui::Ui| self.view(ui)
    }

    pub fn toggle_orientation(&mut self) {
        self.reversed = ! self.reversed;
    }

    fn view(&self, ui: &mut Ui) -> egui::Response {
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
            /////////////////////////////////
            println!("Drag started !");
            /////////////////////////////////
        } else if response.drag_released() {
            /////////////////////////////////
            println!("Drag released !");
            /////////////////////////////////
        } else if response.dragged() {
            /////////////////////////////////
            println!("Dragged !");
            /////////////////////////////////
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
            );
            painter::draw_coordinates(ui, rect, self.reversed);
            painter::draw_player_turn(ui, rect, self.position.clone());
        }
        response
    }
}
