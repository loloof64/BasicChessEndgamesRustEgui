use eframe::{
    egui::{self, ImageButton, Sense},
    epaint::{Mesh, Vec2},
};
use gui::chessboard::ChessBoard;

mod gui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Basic chess endgames",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    board: ChessBoard,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            board: ChessBoard::new(500.0),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.vertical_centered(|ui| {
                    let buttons_size = 50.0;
                    let reverse_image = egui_extras::RetainedImage::from_svg_bytes(
                        "reverse",
                        include_bytes!("./gui/assets/images/reverse.svg"),
                    )
                    .unwrap();
                    let reverse_board_button = ImageButton::new(
                        reverse_image.texture_id(ctx),
                        Vec2 {
                            x: buttons_size,
                            y: buttons_size,
                        },
                    );
                    if ui.add(reverse_board_button).clicked() {
                        self.board.toggle_orientation();
                    };
                });
                ui.vertical_centered(|ui| {
                    ui.add(self.board.widget());
                });
            });
        });
    }
}
