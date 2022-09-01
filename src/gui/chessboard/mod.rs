use eframe::{
    egui::{self, Ui},
};

mod painter;

pub struct ChessBoard {
    size: f32,
}

impl ChessBoard {
    pub fn new(size: f32) -> Self {
        Self { size }
    }

    pub fn widget(&self) -> impl egui::Widget + '_ {
        move |ui: &mut egui::Ui| self.view(ui)
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
        }
        response
    }
}
