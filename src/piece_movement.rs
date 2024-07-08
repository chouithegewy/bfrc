#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::piece::{Color, Move, MoveType, Piece, PieceType, Position};

struct Direction {
    x: i8,
    y: i8,
}

impl Direction {
    fn new(x: i8, y: i8) -> Direction {
        Direction { x, y }
    }
}

pub fn generate_move_set(piece: Piece) -> Vec<Move> {
    match piece.piece_type {
        PieceType::King(_) => generate_king_moves(piece),
        PieceType::Queen(_) => generate_queen_moves(piece),
        PieceType::Rook(_) => generate_rook_moves(piece),
        PieceType::Bishop(_) => generate_bishop_moves(piece),
        PieceType::Knight(_) => generate_knight_moves(piece),
        PieceType::Pawn(_) => generate_pawn_moves(piece),
        PieceType::Empty => vec![],
    }
}

fn generate_king_moves(piece: Piece) -> Vec<Move> {
    let mut move_set = vec![];
    let directions = vec![
        Direction::new(1, 0),
        Direction::new(1, 1),
        Direction::new(0, 1),
        Direction::new(-1, 1),
        Direction::new(-1, 0),
        Direction::new(-1, -1),
        Direction::new(0, -1),
        Direction::new(1, -1),
    ];
    for direction in directions {
        let new_pos = Position::new(
            piece.position.col + direction.x,
            piece.position.row + direction.y,
        );
        while new_pos.is_valid() {
            move_set.push(Move {
                start_pos: None,
                end_pos: new_pos,
                piece_type: piece.piece_type,
                captures: false,
                move_type: MoveType::Normal,
                check: false,
            });
        }
    }
    move_set
}

fn generate_queen_moves(piece: Piece) -> Vec<Move> {
    let mut move_set = vec![];
    move_set.append(&mut generate_rook_moves(piece.clone()));
    move_set.append(&mut generate_bishop_moves(piece.clone()));
    move_set
}

fn generate_rook_moves(piece: Piece) -> Vec<Move> {
    let mut move_set = vec![];
    let directions = vec![
        Direction::new(1, 0),
        Direction::new(0, 1),
        Direction::new(-1, 0),
        Direction::new(0, -1),
    ];
    for direction in directions {
        let mut new_pos_col = Position::new(piece.position.col + direction.x, piece.position.row);
        let mut new_pos_row = Position::new(piece.position.col, piece.position.row + direction.y);
        while new_pos_col.is_valid() {
            move_set.push(Move {
                start_pos: None,
                end_pos: new_pos_col,
                piece_type: piece.piece_type,
                captures: false,
                move_type: MoveType::Normal,
                check: false,
            });
            new_pos_col = Position::new(new_pos_col.col + direction.x, new_pos_col.row);
        }
        while new_pos_row.is_valid() {
            move_set.push(Move {
                start_pos: None,
                end_pos: new_pos_row,
                piece_type: piece.piece_type,
                captures: false,
                move_type: MoveType::Normal,
                check: false,
            });
            new_pos_row = Position::new(new_pos_row.col, new_pos_row.row + direction.y);
        }
    }
    move_set
}

fn generate_bishop_moves(piece: Piece) -> Vec<Move> {
    let mut move_set = vec![];
    let directions = vec![
        Direction::new(1, 1),
        Direction::new(-1, 1),
        Direction::new(-1, -1),
        Direction::new(1, -1),
    ];
    for direction in directions {
        let mut new_pos = Position::new(
            piece.position.col + direction.x,
            piece.position.row + direction.y,
        );
        while new_pos.is_valid() {
            move_set.push(Move {
                start_pos: None,
                end_pos: new_pos,
                piece_type: piece.piece_type,
                captures: false,
                move_type: MoveType::Normal,
                check: false,
            });
            new_pos = Position::new(new_pos.col + direction.x, new_pos.row + direction.y);
        }
    }
    move_set
}

fn generate_knight_moves(piece: Piece) -> Vec<Move> {
    let mut move_set = vec![];
    let directions = vec![
        Direction::new(1, 2),
        Direction::new(2, 1),
        Direction::new(2, -1),
        Direction::new(1, -2),
        Direction::new(-1, -2),
        Direction::new(-2, -1),
        Direction::new(-2, 1),
        Direction::new(-1, 2),
    ];
    for direction in directions {
        let new_pos = Position::new(
            piece.position.col + direction.x,
            piece.position.row + direction.y,
        );
        move_set.push(Move {
            start_pos: None,
            end_pos: new_pos,
            piece_type: piece.piece_type,
            captures: false,
            move_type: MoveType::Normal,
            check: false,
        });
    }
    move_set
}

fn generate_pawn_moves(piece: Piece) -> Vec<Move> {
    let mut move_set = vec![];
    let directions = vec![
        Direction::new(0, 1),
        Direction::new(0, 2),
        Direction::new(1, 1),
        Direction::new(-1, 1),
    ];
    for direction in directions {
        let new_pos = Position::new(
            piece.position.col + direction.x,
            piece.position.row + direction.y,
        );
        move_set.push(Move {
            start_pos: None,
            end_pos: new_pos,
            piece_type: piece.piece_type,
            captures: false,
            move_type: MoveType::Normal,
            check: false,
        });
    }
    move_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_move_set() {
        let piece = Piece::new(PieceType::King(Color::White), Position::new(4, 4));
        let move_set = generate_move_set(piece);
        assert_eq!(move_set.len(), 8);
    }

    #[test]
    fn test_generate_king_moves() {
        let piece = Piece::new(PieceType::King(Color::White), Position::new(4, 4));
        let move_set = generate_king_moves(piece);
        assert_eq!(move_set.len(), 8);
    }

    #[test]
    fn test_generate_queen_moves() {
        let piece = Piece::new(PieceType::Queen(Color::White), Position::new(4, 4));
        let move_set = generate_queen_moves(piece);
        assert_eq!(move_set.len(), 28);
    }

    #[test]
    fn test_generate_rook_moves() {
        let piece = Piece::new(PieceType::Rook(Color::White), Position::new(4, 4));
        let move_set = generate_rook_moves(piece);
        assert_eq!(move_set.len(), 14);
    }

    #[test]
    fn test_generate_bishop_moves() {
        let piece = Piece::new(PieceType::Bishop(Color::White), Position::new(4, 4));
        let move_set = generate_bishop_moves(piece);
        assert_eq!(move_set.len(), 13);
    }

    #[test]
    fn test_generate_knight_moves() {
        let piece = Piece::new(PieceType::Knight(Color::White), Position::new(4, 4));
        let move_set = generate_knight_moves(piece);
        assert_eq!(move_set.len(), 8);
    }

    #[test]
    fn test_generate_pawn_moves() {
        let piece = Piece::new(PieceType::Pawn(Color::White), Position::new(4, 4));
        let move_set = generate_pawn_moves(piece);
        assert_eq!(move_set.len(), 4);
    }
}
