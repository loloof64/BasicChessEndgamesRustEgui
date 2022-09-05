use core::ascii;
use eframe::{
    egui::Ui,
    epaint::{Color32, FontId, Mesh, Pos2, Rect, RectShape, Rounding, Shape},
};
use owlchess::{Board, Color, File, Rank};

use super::{pieces_images::PiecesImages, utils::get_piece_type_from, DndData};

pub(crate) fn draw_background(ui: &mut Ui, rect: Rect) {
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

pub(crate) fn draw_cells(ui: &mut Ui, rect: Rect, reversed: bool, dnd_data: &Option<DndData>) {
    let size = rect.size().x;
    let cells_size = size * 0.111;

    for row in 0..=7 {
        for col in 0..=7 {
            let file = (if reversed { 7 - col } else { col }) as u8;
            let rank = (if reversed { 7 - row } else { row }) as u8;

            let white_cell = (col + row) % 2 == 0;

            let is_dnd_start_cell = match dnd_data {
                Some(DndData {
                    start_file,
                    start_rank,
                    ..
                }) => file == *start_file && rank == *start_rank,
                None => false,
            };

            let is_dnd_end_cell = match dnd_data {
                Some(DndData {
                    end_file, end_rank, ..
                }) => file == *end_file && rank == *end_rank,
                None => false,
            };

            let is_dnd_cross_cell = match dnd_data {
                Some(DndData {
                    end_file, end_rank, ..
                }) => file == *end_file || rank == *end_rank,
                None => false,
            };

            let mut color = if white_cell {
                Color32::from_rgb(255, 222, 173)
            } else {
                Color32::from_rgb(205, 133, 63)
            };
            if is_dnd_start_cell {
                color = Color32::from_rgb(205, 92, 92);
            }
            if is_dnd_cross_cell {
                color = Color32::from_rgb(255, 182, 193);
            }
            if is_dnd_end_cell {
                color = Color32::from_rgb(50, 205, 50);
            }

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

pub(crate) fn draw_pieces(
    ui: &mut Ui,
    rect: Rect,
    pieces_images: &PiecesImages,
    board_value: Board,
    reversed: bool,
    dnd_data: &Option<DndData>,
) {
    let size = rect.size().x;
    let cells_size = size * 0.111;

    for row in 0..=7 {
        for col in 0..=7 {
            let file = if reversed { 7 - col } else { col };
            let rank = if reversed { row } else { 7 - row };

            let is_moved_piece = match dnd_data {
                Some(DndData {
                    start_file,
                    start_rank,
                    ..
                }) => *start_file == file && *start_rank == rank,
                _ => false,
            };

            if is_moved_piece {
                continue;
            }

            let x = rect.min.x + cells_size * (0.5 + col as f32);
            let y = rect.min.y + cells_size * (7.5 - row as f32);

            let piece_rect = Rect {
                min: Pos2 { x, y },
                max: Pos2 {
                    x: x + cells_size,
                    y: y + cells_size,
                },
            };

            let square = board_value.get2(
                File::from_index(file as usize),
                Rank::from_index(rank as usize),
            );
            if square.is_free() {
                continue;
            }

            let piece_type = square.piece();
            let piece_color = square.color();
            let piece_type = get_piece_type_from(piece_type.unwrap(), piece_color.unwrap());

            let image = match piece_type {
                'P' => &pieces_images.wp,
                'N' => &pieces_images.wn,
                'B' => &pieces_images.wb,
                'R' => &pieces_images.wr,
                'Q' => &pieces_images.wq,
                'K' => &pieces_images.wk,
                'p' => &pieces_images.bp,
                'n' => &pieces_images.bn,
                'b' => &pieces_images.bb,
                'r' => &pieces_images.br,
                'q' => &pieces_images.bq,
                'k' => &pieces_images.bk,
                _ => panic!("Not recognized piece {}", piece_type),
            };

            {
                let ctx = ui.ctx();
                let mut mesh = Mesh::with_texture(image.texture_id(ctx));
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

pub(crate) fn draw_coordinates(ui: &mut Ui, rect: Rect, reversed: bool) {
    let size = rect.size().x;
    let cells_size = size * 0.111;

    let font_size = cells_size * 0.4;
    let text_color = Color32::from_rgb(255, 220, 10);

    for col in 0..=7 {
        let file = if reversed { 7 - col } else { col };
        let text = (ascii::escape_default(b'A').next().unwrap() + file) as char;
        let text = format!("{}", text);
        let x = rect.min.x + cells_size * (0.90 + col as f32);
        let y1 = rect.min.y + cells_size * 0.05;
        let y2 = rect.min.y + cells_size * 8.55;
        ui.painter().text(
            Pos2 { x, y: y1 },
            eframe::emath::Align2::LEFT_TOP,
            text.clone(),
            FontId::monospace(font_size),
            text_color,
        );
        ui.painter().text(
            Pos2 { x, y: y2 },
            eframe::emath::Align2::LEFT_TOP,
            text.clone(),
            FontId::monospace(font_size),
            text_color,
        );
    }

    for row in 0..=7 {
        let rank = if reversed { row } else { 7 - row };
        let text = (ascii::escape_default(b'1').next().unwrap() + rank) as char;
        let text = format!("{}", text);
        let x1 = rect.min.x + cells_size * 0.15;
        let x2 = rect.min.x + cells_size * 8.65;
        let y = rect.min.y + cells_size * (0.8 + row as f32);
        ui.painter().text(
            Pos2 { x: x1, y },
            eframe::emath::Align2::LEFT_TOP,
            text.clone(),
            FontId::monospace(font_size),
            text_color,
        );
        ui.painter().text(
            Pos2 { x: x2, y },
            eframe::emath::Align2::LEFT_TOP,
            text.clone(),
            FontId::monospace(font_size),
            text_color,
        );
    }
}

pub(crate) fn draw_player_turn(ui: &mut Ui, rect: Rect, board_value: Board) {
    let size = rect.size().x;
    let cells_size = size * 0.111;
    let x = rect.min.x + cells_size * 8.75;
    let y = rect.min.y + cells_size * 8.75;

    let white_turn = board_value.side() == Color::White;
    let color = if white_turn {
        Color32::WHITE
    } else {
        Color32::BLACK
    };
    ui.painter()
        .circle_filled(Pos2 { x, y }, cells_size * 0.25, color);
}

pub(crate) fn draw_moved_piece(
    ui: &mut Ui,
    rect: Rect,
    dnd_data: &Option<DndData>,
    pieces_images: &PiecesImages,
) {
    let size = rect.size().x;
    let cells_size = size * 0.111;

    if let Some(dnd_data) = dnd_data {
        let piece_type = get_piece_type_from(dnd_data.piece_type, dnd_data.piece_color);

        let image = match piece_type {
            'P' => &pieces_images.wp,
            'N' => &pieces_images.wn,
            'B' => &pieces_images.wb,
            'R' => &pieces_images.wr,
            'Q' => &pieces_images.wq,
            'K' => &pieces_images.wk,
            'p' => &pieces_images.bp,
            'n' => &pieces_images.bn,
            'b' => &pieces_images.bb,
            'r' => &pieces_images.br,
            'q' => &pieces_images.bq,
            'k' => &pieces_images.bk,
            _ => panic!("Not recognized piece {}", piece_type),
        };

        {
            let piece_rect = Rect {
                min: Pos2 {
                    x: dnd_data.x - cells_size * 0.5,
                    y: dnd_data.y - cells_size * 0.5,
                },
                max: Pos2 {
                    x: dnd_data.x + cells_size - cells_size * 0.5,
                    y: dnd_data.y + cells_size - cells_size * 0.5,
                },
            };
            let ctx = ui.ctx();
            let mut mesh = Mesh::with_texture(image.texture_id(ctx));
            mesh.add_rect_with_uv(
                piece_rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );
            ui.painter().add(mesh);
        }
    }
}
