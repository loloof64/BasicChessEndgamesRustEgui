use eframe::{
    egui::Ui,
    epaint::{Color32, Pos2, Rect, RectShape, Rounding, Shape},
};

pub fn draw_background(ui: &mut Ui, rect: Rect) {
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

pub fn draw_cells(ui: &mut Ui, rect: Rect) {
    let size = rect.size().x;
    let cells_size = size * 0.111;

    for row in 0..=7 {
        for col in 0..=7 {
            let white_cell = (col + row) % 2 == 0;
            let color = if white_cell {
                Color32::from_rgb(255, 222, 173)
            } else {
                Color32::from_rgb(205, 133, 63)
            };
            let x = cells_size * (0.5 + col as f32) + rect.left();
            let y = cells_size * (0.5 + row as f32) + rect.top();

            let cell_rect = Rect {
                min: Pos2 { x, y },
                max: Pos2 {
                    x: x + cells_size,
                    y: y + cells_size,
                },
            };

            ui.painter().add(Shape::Rect(RectShape {
                rect: cell_rect,
                fill: color,
                rounding: Rounding::none(),
                stroke: eframe::epaint::Stroke {
                    width: 0.0,
                    color: Color32::TRANSPARENT,
                },
            }));
        }
    }
}
