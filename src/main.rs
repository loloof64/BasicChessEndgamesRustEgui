use eframe::{
    egui::{self, ImageButton},
    epaint::{Vec2, Color32},
};
use gui::chessboard::{ChessBoard, Colors};

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
            board: ChessBoard::new(
                500.0,
                Box::new(|move_san|{
                    println!("{}",*move_san);
                })
            ),
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
                    let mut board_colors = Colors::default();
                    board_colors.set_last_move_arrow(Color32::from_rgb(12, 250, 12));
                    board_colors.set_coordinates(Color32::from_rgb(250, 10, 20));
                    board_colors.set_background(Color32::from_rgb(80, 150, 50));
                    self.board.set_colors(board_colors);
                    ui.add(self.board.widget());
                });
            });
        });
    }
}
