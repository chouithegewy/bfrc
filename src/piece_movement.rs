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

pub fn generate_move_set(mv: &Move, board: &Board, color: Color) -> Vec<Move> {
    match mv.piece_type {
        PieceType::King(_) => generate_king_moves(mv, board, color),
        PieceType::Queen(_) => generate_queen_moves(mv, board, color),
        PieceType::Rook(_) => generate_rook_moves(mv, board, color),
        PieceType::Bishop(_) => generate_bishop_moves(mv, board, color),
        PieceType::Knight(_) => generate_knight_moves(mv, board, color),
        PieceType::Pawn(_) => generate_pawn_moves(mv, board, color),
        PieceType::Empty => vec![],
    }
}

fn generate_king_moves(mv: &Move, board: &Board, color: Color) -> Vec<Move> {
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
            mv.start_pos.expect("Failed to unwrap start_pos").row + direction.y,
            mv.start_pos.expect("Failed to unwrap start_pos").col + direction.x,
        );
        if new_pos.is_valid() && Some(color) != board.get_piece_at_position(&new_pos).color() {
            move_set.push(Move {
                start_pos: mv.start_pos,
                end_pos: new_pos,
                piece_type: mv.piece_type,
                captures: false,
                move_type: MoveType::Normal,
                check: false,
            });
        }
    }
    move_set
}

fn generate_queen_moves(mv: &Move, board: &Board, color: Color) -> Vec<Move> {
    let mut move_set = vec![];
    move_set.append(&mut generate_rook_moves(&mv, board, color));
    move_set.append(&mut generate_bishop_moves(&mv, board, color));
    move_set
}

fn generate_rook_moves(mv: &Move, board: &Board, color: Color) -> Vec<Move> {
    let mut move_set = vec![];
    let directions = vec![
        Direction::new(1, 0),
        Direction::new(0, 1),
        Direction::new(-1, 0),
        Direction::new(0, -1),
    ];
    for direction in &directions {
        let mut new_pos = Position::new(
            mv.start_pos.expect("Failed to unwrap start_pos").row + direction.y,
            mv.start_pos.expect("Failed to unwrap start_pos").col + direction.x,
        );
        while new_pos.is_valid() && Some(color) != board.get_piece_at_position(&new_pos).color() {
            move_set.push(Move {
                start_pos: mv.start_pos,
                end_pos: new_pos,
                piece_type: mv.piece_type,
                captures: false,
                move_type: MoveType::Normal,
                check: false,
            });
            new_pos = Position::new(new_pos.row + direction.y, new_pos.col + direction.x);
        }
    }

    move_set
}

fn generate_bishop_moves(mv: &Move, board: &Board, color: Color) -> Vec<Move> {
    let mut move_set = vec![];
    let directions = vec![
        Direction::new(1, 1),
        Direction::new(-1, 1),
        Direction::new(-1, -1),
        Direction::new(1, -1),
    ];
    for direction in directions {
        let mut new_pos = Position::new(
            mv.start_pos.expect("Failed to unwrap start_pos").col + direction.x,
            mv.start_pos.expect("Failed to unwrap start_pos").row + direction.y,
        );
        while new_pos.is_valid() && Some(color) != board.get_piece_at_position(&new_pos).color() {
            move_set.push(Move {
                start_pos: mv.start_pos,
                end_pos: new_pos,
                piece_type: mv.piece_type,
                captures: false,
                move_type: MoveType::Normal,
                check: false,
            });
            new_pos = Position::new(new_pos.row + direction.y, new_pos.col + direction.x);
        }
    }
    move_set
}

fn generate_knight_moves(mv: &Move, board: &Board, color: Color) -> Vec<Move> {
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
            mv.start_pos.expect("Failed to unwrap start_pos").row + direction.y,
            mv.start_pos.expect("Failed to unwrap start_pos").col + direction.x,
        );

        dbg!(new_pos.as_str());

        if new_pos.is_valid() && Some(color) != board.get_piece_at_position(&new_pos).color() {
            move_set.push(Move {
                start_pos: mv.start_pos,
                end_pos: new_pos,
                piece_type: mv.piece_type,
                captures: false,
                move_type: MoveType::Normal,
                check: false,
            });
        }
    }
    move_set
}

fn generate_pawn_moves(mv: &Move, board: &Board, color: Color) -> Vec<Move> {
    let mut move_set = vec![];
    let directions = match mv.captures {
        false => {
            vec![
                // black
                Direction::new(0, 1),
                Direction::new(0, 2), // if piece on starting rank
                // white
                Direction::new(0, -1),
                Direction::new(0, -2), // if piece on starting rank
            ]
        }
        true => {
            vec![
                Direction::new(1, 1),
                Direction::new(-1, 1),
                Direction::new(1, -1),
                Direction::new(-1, -1),
            ]
        }
    };

    for direction in directions {
        let new_pos = Position::new(
            mv.start_pos.expect("Failed to unwrap start_pos").row + direction.y,
            mv.start_pos.expect("Failed to unwrap start_pos").col + direction.x,
        );
        if direction.y == 2 {
            match color {
                Color::White => {
                    if new_pos.row != 6 {
                        continue;
                    }
                }
                Color::Black => {
                    if new_pos.row != 1 {
                        continue;
                    }
                }
            }
        }
        //println!("{}", new_pos.as_str());
        if new_pos.is_valid() && Some(color) != board.get_piece_at_position(&new_pos).color() {
            move_set.push(Move {
                start_pos: mv.start_pos,
                end_pos: new_pos,
                piece_type: mv.piece_type,
                captures: false,
                move_type: MoveType::Normal,
                check: false,
            });
        }
    }
    move_set
}
