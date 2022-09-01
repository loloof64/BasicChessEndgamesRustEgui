use eframe::egui;
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
            board: ChessBoard::new(300.0),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.add(self.board.widget());
            });
        });
    }
}