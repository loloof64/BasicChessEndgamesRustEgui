use owlchess::{Color, Cell, Coord, File, Piece, Rank};
use owlchess::moves::PromotePiece;
use owlchess::moves::uci;

pub(crate) fn get_piece_type_from(piece: Piece, color: Color) -> char {
    Cell::from_parts(color, piece).as_char()
}

pub(crate) fn get_uci_move_for(
    start_file: u8,
    start_rank: u8,
    end_file: u8,
    end_rank: u8,
    promotion: Option<char>,
) -> uci::Move {
    uci::Move::Move {
        src: parse_square_coords(start_file, start_rank),
        dst: parse_square_coords(end_file, end_rank),
        promote: parse_promotion(promotion),
    }
}

fn parse_square_coords(file: u8, rank: u8) -> Coord {
    let file = File::from_index(file as usize);
    let rank = Rank::from_index(7-rank as usize);

    Coord::from_parts(file, rank)
}

fn parse_promotion(piece: Option<char>) -> Option<PromotePiece> {
    Some(match piece? {
        'q' => PromotePiece::Queen,
        'r' => PromotePiece::Rook,
        'b' => PromotePiece::Bishop,
        'n' => PromotePiece::Knight,
        other => panic!("Forbidden promote value : {}.", other),
    })
}

pub fn san_to_fan(move_san: String, white_move: bool) -> String {
    let pieces_refs = String::from("NBRQK");
    let first_piece_occurence = move_san.chars().position(|elem| pieces_refs.contains(elem));
    match first_piece_occurence {
        Some(index) => {
            let occurence = move_san.chars().nth(index).unwrap();
            let replacement = match occurence {
                'N' => if white_move {'\u{2658}'} else {'\u{265e}'}, 
                'B' => if white_move {'\u{2657}'} else {'\u{265d}'}, 
                'R' => if white_move {'\u{2656}'} else {'\u{265c}'}, 
                'Q' => if white_move {'\u{2655}'} else {'\u{265b}'}, 
                'K' => if white_move {'\u{2654}'} else {'\u{265a}'},
                _ => occurence, 
            };
            let first_part: String = move_san.chars().take(index).collect();
            let last_part: String = move_san.chars().skip(index+1).collect();
            format!("{}{}{}", first_part, replacement, last_part)
        },
        _ => move_san
    }
}
