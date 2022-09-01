use eframe::{
    egui::{self, Ui},
    epaint::{Color32, RectShape, Rounding, Shape},
};

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
        }

        // TODO Attach some meta-data to the response which can be used by screen readers:
        // response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

        // 4. Paint!
        // Make sure we need to paint:
        if ui.is_rect_visible(rect) {
            ui.painter().add(Shape::Rect(RectShape {
                rect,
                fill: Color32::from_rgb(35, 136, 210),
                rounding: Rounding::none(),
                stroke: eframe::epaint::Stroke {
                    width: 0.0,
                    color: Color32::TRANSPARENT,
                },
            }));
        }
        response
    }
}
