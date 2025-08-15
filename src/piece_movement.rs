#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::{
    board::Board,
    piece::{Color, Move, MoveType, Piece, PieceType, Position},
};

#[derive(Debug, Clone, Copy)]
struct Direction {
    x: i8,
    y: i8,
}

impl Direction {
    fn new(x: i8, y: i8) -> Direction {
        Direction { x, y }
    }
}

pub fn generate_move_set(piece: &Piece, board: &Board) -> Vec<Move> {
    match piece.piece_type {
        PieceType::King(_) => generate_king_moves(piece, board),
        PieceType::Queen(_) => generate_queen_moves(piece, board),
        PieceType::Rook(_) => generate_rook_moves(piece, board),
        PieceType::Bishop(_) => generate_bishop_moves(piece, board),
        PieceType::Knight(_) => generate_knight_moves(piece, board),
        PieceType::Pawn(_) => generate_pawn_moves(piece, board),
        PieceType::Empty => vec![],
    }
}

fn generate_king_moves(piece: &Piece, board: &Board) -> Vec<Move> {
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
            piece.position.row + direction.y,
            piece.position.col + direction.x,
        );
        if new_pos.is_valid() && piece.is_empty_or_not_same_color(&new_pos, board) {
            move_set.push(Move {
                start_pos: Some(piece.position),
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

fn generate_queen_moves(piece: &Piece, board: &Board) -> Vec<Move> {
    let mut move_set = vec![];
    move_set.append(&mut generate_rook_moves(&piece, board));
    move_set.append(&mut generate_bishop_moves(&piece, board));
    move_set
}

fn generate_rook_moves(piece: &Piece, board: &Board) -> Vec<Move> {
    let mut move_set = vec![];
    let directions = vec![
        Direction::new(1, 0),
        Direction::new(0, 1),
        Direction::new(-1, 0),
        Direction::new(0, -1),
    ];
    for direction in &directions {
        let mut new_pos = Position::new(
            piece.position.row + direction.y,
            piece.position.col + direction.x,
        );
        while new_pos.is_valid() && piece.is_empty_or_not_same_color(&new_pos, board) {
            move_set.push(Move {
                start_pos: Some(piece.position),
                end_pos: new_pos,
                piece_type: piece.piece_type,
                captures: false,
                move_type: MoveType::Normal,
                check: false,
            });
            new_pos = Position::new(new_pos.row + direction.y, new_pos.col + direction.x);
        }
    }

    move_set
}

fn generate_bishop_moves(piece: &Piece, board: &Board) -> Vec<Move> {
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
        while new_pos.is_valid() && piece.is_empty_or_not_same_color(&new_pos, board) {
            move_set.push(Move {
                start_pos: Some(piece.position),
                end_pos: new_pos,
                piece_type: piece.piece_type,
                captures: false,
                move_type: MoveType::Normal,
                check: false,
            });
            new_pos = Position::new(new_pos.row + direction.y, new_pos.col + direction.x);
        }
    }
    move_set
}

fn generate_knight_moves(piece: &Piece, board: &Board) -> Vec<Move> {
    let mut move_set = vec![];
    let directions = vec![
        Direction::new(1, 2),
        Direction::new(1, -2),
        Direction::new(2, 1),
        Direction::new(2, -1),
        Direction::new(-1, -2),
        Direction::new(-1, 2),
        Direction::new(-2, -1),
        Direction::new(-2, 1),
    ];
    for direction in directions {
        let new_pos = Position::new(
            piece.position.row + direction.y,
            piece.position.col + direction.x,
        );

        if new_pos.is_valid() && piece.is_empty_or_not_same_color(&new_pos, board) {
            move_set.push(Move {
                start_pos: Some(piece.position),
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

fn generate_pawn_moves(piece: &Piece, board: &Board) -> Vec<Move> {
    let mut move_set = vec![];
    let directions = vec![
        Direction::new(0, 1),
        Direction::new(0, 2),
        Direction::new(1, 1),
        Direction::new(-1, 1),
    ];
    for direction in directions {
        let new_pos = Position::new(
            piece.position.row + direction.y,
            piece.position.col + direction.x,
        );
        if new_pos.is_valid() && piece.is_empty_or_not_same_color(&new_pos, board) {
            move_set.push(Move {
                start_pos: Some(piece.position),
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
