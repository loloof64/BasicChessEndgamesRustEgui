use core::ascii;
use eframe::{
    egui::{ImageButton, Ui},
    epaint::{
        Color32, FontId, Mesh, Pos2, Rect, RectShape, Rounding, Shape, Stroke, Vec2,
    },
};
use owlchess::{Color, File, Rank};
use std::ops::Add;

use super::{utils::get_piece_type_from, ChessBoard, DndData, Colors};

pub(crate) fn draw_background(ui: &mut Ui, rect: Rect, colors: &Colors) {
    ui.painter().add(Shape::Rect(RectShape {
        rect,
        fill: colors.background,
        rounding: Rounding::none(),
        stroke: eframe::epaint::Stroke {
            width: 0.0,
            color: Color32::TRANSPARENT,
        },
    }));
}

pub(crate) fn draw_cells(ui: &mut Ui, rect: Rect, board: &ChessBoard) {
    let size = rect.size().x;
    let cells_size = size * 0.111;

    for row in 0..=7 {
        for col in 0..=7 {
            let file = (if board.reversed { 7 - col } else { col }) as u8;
            let rank = (if board.reversed { 7 - row } else { row }) as u8;

            let white_cell = (col + row) % 2 == 0;

            let is_dnd_start_cell = match board.dnd_data {
                Some(DndData {
                    start_file,
                    start_rank,
                    ..
                }) => file == start_file && rank == start_rank,
                None => false,
            };

            let is_dnd_end_cell = match board.dnd_data {
                Some(DndData {
                    end_file, end_rank, ..
                }) => file == end_file && rank == end_rank,
                None => false,
            };

            let is_dnd_cross_cell = match board.dnd_data {
                Some(DndData {
                    end_file, end_rank, ..
                }) => file == end_file || rank == end_rank,
                None => false,
            };

            let mut color = if white_cell {
                board.colors.white_cells
            } else {
                board.colors.black_cells
            };
            if is_dnd_start_cell {
                color = board.colors.dnd_start_cell;
            }
            if is_dnd_cross_cell {
                color = board.colors.dnd_cross_cell;
            }
            if is_dnd_end_cell {
                color = board.colors.dnd_end_cell;
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

pub(crate) fn draw_pieces(ui: &mut Ui, rect: Rect, board: &ChessBoard) {
    let size = rect.size().x;
    let cells_size = size * 0.111;

    for row in 0..=7 {
        for col in 0..=7 {
            let file = if board.reversed { 7 - col } else { col };
            let rank = if board.reversed { row } else { 7 - row };

            let is_moved_piece = match board.dnd_data {
                Some(DndData {
                    start_file,
                    start_rank,
                    ..
                }) => start_file == file && start_rank == rank,
                _ => false,
            };

            if is_moved_piece {
                continue;
            }

            let x = rect.min.x + cells_size * (0.5 + col as f32);
            let y = rect.min.y + cells_size * (0.5 + row as f32);

            let piece_rect = Rect {
                min: Pos2 { x, y },
                max: Pos2 {
                    x: x + cells_size,
                    y: y + cells_size,
                },
            };

            let square = board.position.get2(
                File::from_index(file as usize),
                Rank::from_index(7 - rank as usize),
            );
            if square.is_free() {
                continue;
            }

            let piece_type = square.piece();
            let piece_color = square.color();
            let piece_type = get_piece_type_from(piece_type.unwrap(), piece_color.unwrap());

            let image = match piece_type {
                'P' => &board.pieces_images.wp,
                'N' => &board.pieces_images.wn,
                'B' => &board.pieces_images.wb,
                'R' => &board.pieces_images.wr,
                'Q' => &board.pieces_images.wq,
                'K' => &board.pieces_images.wk,
                'p' => &board.pieces_images.bp,
                'n' => &board.pieces_images.bn,
                'b' => &board.pieces_images.bb,
                'r' => &board.pieces_images.br,
                'q' => &board.pieces_images.bq,
                'k' => &board.pieces_images.bk,
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

pub(crate) fn draw_coordinates(ui: &mut Ui, rect: Rect, board: &ChessBoard) {
    let size = rect.size().x;
    let cells_size = size * 0.111;

    let font_size = cells_size * 0.4;
    let text_color = board.colors.coordinates;

    for col in 0..=7 {
        let file = if board.reversed { 7 - col } else { col };
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
        let rank = if board.reversed { row } else { 7 - row };
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

pub(crate) fn draw_player_turn(ui: &mut Ui, rect: Rect, board: &ChessBoard) {
    let size = rect.size().x;
    let cells_size = size * 0.111;
    let x = rect.min.x + cells_size * 8.75;
    let y = rect.min.y + cells_size * 8.75;

    let white_turn = board.position.side() == Color::White;
    let color = if white_turn {
        Color32::WHITE
    } else {
        Color32::BLACK
    };
    ui.painter()
        .circle_filled(Pos2 { x, y }, cells_size * 0.25, color);
}

pub(crate) fn draw_moved_piece(ui: &mut Ui, rect: Rect, board: &ChessBoard) {
    let size = rect.size().x;
    let cells_size = size * 0.111;

    if let Some(dnd_data) = &board.dnd_data {
        let piece_type = get_piece_type_from(dnd_data.piece_type, dnd_data.piece_color);

        let image = match piece_type {
            'P' => &board.pieces_images.wp,
            'N' => &board.pieces_images.wn,
            'B' => &board.pieces_images.wb,
            'R' => &board.pieces_images.wr,
            'Q' => &board.pieces_images.wq,
            'K' => &board.pieces_images.wk,
            'p' => &board.pieces_images.bp,
            'n' => &board.pieces_images.bn,
            'b' => &board.pieces_images.bb,
            'r' => &board.pieces_images.br,
            'q' => &board.pieces_images.bq,
            'k' => &board.pieces_images.bk,
            _ => panic!("Not recognized piece {}", piece_type),
        };

        {
            let piece_rect = if (board.reversed != dnd_data.started_with_reversed_board)
                && dnd_data.has_pending_promotion
            {
                let center = rect.center();
                let dnd_position = Pos2 {
                    x: dnd_data.x,
                    y: dnd_data.y,
                };
                let half_distance = center - dnd_position;
                let new_position = center + half_distance;
                Rect {
                    min: Pos2 {
                        x: new_position.x - cells_size * 0.5,
                        y: new_position.y - cells_size * 0.5,
                    },
                    max: Pos2 {
                        x: new_position.x + cells_size - cells_size * 0.5,
                        y: new_position.y + cells_size - cells_size * 0.5,
                    },
                }
            } else {
                Rect {
                    min: Pos2 {
                        x: dnd_data.x - cells_size * 0.5,
                        y: dnd_data.y - cells_size * 0.5,
                    },
                    max: Pos2 {
                        x: dnd_data.x + cells_size - cells_size * 0.5,
                        y: dnd_data.y + cells_size - cells_size * 0.5,
                    },
                }
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

pub(crate) fn draw_promotion_buttons(ui: &mut Ui, rect: Rect, board: &mut ChessBoard) {
    // There must be a pending promotion
    match &board.dnd_data {
        Some(dnd_data) => {
            if !dnd_data.has_pending_promotion {
                return;
            }
        }
        _ => return,
    }

    let size = rect.size().x;
    let cells_size = size * 0.111;
    let buttons_size = cells_size * 1.5;

    let white_turn = board.position.side() == Color::White;

    let buttons_at_bottom = (board.reversed && !white_turn) || (!board.reversed && white_turn);
    let buttons_bar_x = rect.min.x + cells_size * 0.8;
    let buttons_bar_y = rect.min.y
        + (if buttons_at_bottom {
            cells_size * 6.8
        } else {
            cells_size * 0.8
        });
    let queen_button_x = buttons_bar_x;
    let rook_button_x = rect.min.x + cells_size * 2.8;
    let bishop_button_x = rect.min.x + cells_size * 4.8;
    let knight_button_x = rect.min.x + cells_size * 6.8;

    let buttons_size = Vec2 {
        x: buttons_size,
        y: buttons_size,
    };
    let queen_button_pos = Pos2 {
        x: queen_button_x,
        y: buttons_bar_y,
    };
    let rook_button_pos = Pos2 {
        x: rook_button_x,
        y: buttons_bar_y,
    };
    let bishop_button_pos = Pos2 {
        x: bishop_button_x,
        y: buttons_bar_y,
    };
    let knight_button_pos = Pos2 {
        x: knight_button_x,
        y: buttons_bar_y,
    };

    let queen_image = if white_turn {
        &board.pieces_images.wq
    } else {
        &board.pieces_images.bq
    };
    let rook_image = if white_turn {
        &board.pieces_images.wr
    } else {
        &board.pieces_images.br
    };
    let bishop_image = if white_turn {
        &board.pieces_images.wb
    } else {
        &board.pieces_images.bb
    };
    let knight_image = if white_turn {
        &board.pieces_images.wn
    } else {
        &board.pieces_images.bn
    };

    let queen_button_rect = Rect {
        min: queen_button_pos,
        max: queen_button_pos + buttons_size,
    };

    let rook_button_rect = Rect {
        min: rook_button_pos,
        max: rook_button_pos + buttons_size,
    };

    let bishop_button_rect = Rect {
        min: bishop_button_pos,
        max: bishop_button_pos + buttons_size,
    };

    let knight_button_rect = Rect {
        min: knight_button_pos,
        max: knight_button_pos + buttons_size,
    };
    {
        let ctx = ui.ctx();
        let queen_button = ImageButton::new(queen_image.texture_id(ctx), buttons_size);
        let rook_button = ImageButton::new(rook_image.texture_id(ctx), buttons_size);
        let bishop_button = ImageButton::new(bishop_image.texture_id(ctx), buttons_size);
        let knight_button = ImageButton::new(knight_image.texture_id(ctx), buttons_size);

        if ui.put(queen_button_rect, queen_button).clicked() {
            board.commit_promotion('q');
        }

        if ui.put(rook_button_rect, rook_button).clicked() {
            board.commit_promotion('r');
        }

        if ui.put(bishop_button_rect, bishop_button).clicked() {
            board.commit_promotion('b');
        }

        if ui.put(knight_button_rect, knight_button).clicked() {
            board.commit_promotion('n');
        }
    }
}

pub(crate) fn draw_last_move_arrow(ui: &mut Ui, rect: Rect, board: &ChessBoard) {
    match &board.last_move_arrow {
        Some(arrow_coords) => draw_arrow(ui, rect, arrow_coords, board.reversed, board.colors.last_move_arrow),
        _ => {}
    }
}

fn draw_arrow(ui: &mut Ui, rect: Rect, arrow: &(u8, u8, u8, u8), reversed: bool, color: Color32) {
    let size = rect.size().x;
    let cells_size = size * 0.111;

    let block_size = cells_size;
    let half_block_size = block_size / 2.0;

    let (start_file, start_rank, end_file, end_rank) = arrow;
    let (start_col, start_row, end_col, end_row) = if reversed {
        (
            7 - *start_file,
            7 - *start_rank,
            7 - *end_file,
            7 - *end_rank,
        )
    } else {
        (*start_file, *start_rank, *end_file, *end_rank)
    };

    let start_offset = (
        (((start_col + 1) as f32) * block_size) - half_block_size,
        (((start_row + 1) as f32) * block_size) - half_block_size,
    );
    let end_offset = (
        (((end_col + 1) as f32) * block_size) - half_block_size,
        (((end_row + 1) as f32) * block_size) - half_block_size,
    );

    let arrow_length = f32::sqrt(
        f32::powf(end_offset.0 - start_offset.0, 2f32)
            + f32::powf(end_offset.1 - start_offset.1, 2f32),
    ) * 0.4;
    let arrow_angle = f32::atan2(end_offset.1 - start_offset.1, end_offset.0 - start_offset.0);
    let arrow_point_1 = (
        end_offset.0 - arrow_length * f32::cos(arrow_angle + std::f32::consts::PI / 6f32),
        end_offset.1 - arrow_length * f32::sin(arrow_angle + std::f32::consts::PI / 6f32),
    );
    let arrow_point_2 = (
        end_offset.0 - arrow_length * f32::cos(arrow_angle - std::f32::consts::PI / 6f32),
        end_offset.1 - arrow_length * f32::sin(arrow_angle - std::f32::consts::PI / 6f32),
    );
    let component_offset = Vec2::new(rect.min.x + cells_size * 0.5, rect.min.y + cells_size * 0.5);
    ui.painter().add(Shape::LineSegment {
        points: [
            Pos2::new(start_offset.0, start_offset.1).add(component_offset),
            Pos2::new(end_offset.0, end_offset.1).add(component_offset),
        ],
        stroke: Stroke::new(half_block_size * 0.150, color),
    });
    ui.painter().add(Shape::LineSegment {
        points: [
            Pos2::new(end_offset.0,end_offset.1).add(component_offset),
            Pos2::new(arrow_point_1.0, arrow_point_1.1).add(component_offset),
        ],
        stroke: Stroke::new(half_block_size * 0.150, color),
    });
    ui.painter().add(Shape::LineSegment {
        points: [
            Pos2::new(end_offset.0,end_offset.1).add(component_offset),
            Pos2::new(arrow_point_2.0, arrow_point_2.1).add(component_offset),
        ],
        stroke: Stroke::new(half_block_size * 0.150, color),
    });
}
