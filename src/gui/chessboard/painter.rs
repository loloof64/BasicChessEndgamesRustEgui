use eframe::{
    egui::{TextureFilter, Ui},
    epaint::{Color32, ImageData, Mesh, Pos2, Rect, RectShape, Rounding, Shape},
};

use super::pieces_images::PiecesImages;

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

pub(crate) fn draw_pieces(ui: &mut Ui, rect: Rect, pieces_images: &PiecesImages) {
    let size = rect.size().x;
    let cells_size = size * 0.111;

    for row in 0..=7 {
        for col in 0..=7 {
            let x = rect.min.x + cells_size * (0.5 + col as f32);
            let y = rect.min.y + cells_size * (0.5 + row as f32);

            let piece_rect = Rect {
                min: Pos2 { x, y },
                max: Pos2 {
                    x: x + cells_size,
                    y: y + cells_size,
                },
            };

            {
                let ctx = ui.ctx();
                let mut mesh = Mesh::with_texture(pieces_images.wn.texture_id(ctx));
                mesh.add_rect_with_uv(
                    piece_rect,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    Color32::WHITE,
                );
                ui.painter().add(mesh);
            }
        }
    }
}
