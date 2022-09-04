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
    let rank = Rank::from_index(rank as usize);

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
