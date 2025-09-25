#![allow(dead_code)]
#![allow(unused_variables)]

use crate::board::Board;
// struct Move
// has start?
// has end position
// has piece type
// has move type (capture, check, checkmate, etc)
#[derive(Debug, Clone)]
pub struct Move {
    pub start_pos: Option<Position>,
    pub end_pos: Position,
    pub piece_type: PieceType,
    pub captures: bool,
    pub move_type: MoveType,
    pub check: bool,
}

// enum PieceType
// King, Queen, Rook, Bishop, Knight, Pawn, Empty
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
    Empty,
}

// enum MoveType
// Capture, Check, Checkmate, Castling, Promotion, Normal
#[derive(Debug, Clone, Copy)]
pub enum MoveType {
    Checkmate,
    Castling,
    Promotion,
    Normal,
}

// struct Piece
// has piece type
// has color
// has position
// has move history
// has move set
// has move set generator
// has move set filter
// has move set validator
#[derive(Debug, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub position: Position,
    pub move_history: Vec<Move>,
    pub move_set: Vec<Move>,
}

impl Piece {
    pub fn new(piece_type: PieceType, position: Position) -> Piece {
        Piece {
            piece_type,
            position,
            move_history: Vec::new(),
            move_set: Vec::new(),
        }
    }

    pub fn set_legal_moves(&mut self) {
        match self.piece_type {
            PieceType::King(color) => {}
            PieceType::Queen(color) => {}
            PieceType::Bishop(color) => {}
            PieceType::Rook(color) => {}
            PieceType::Pawn(color) => {}
            PieceType::Knight(color) => {}
            _ => {}
        }
    }

    pub fn color(&self) -> Option<Color> {
        match self.piece_type {
            PieceType::Knight(color) => Some(color),
            PieceType::King(color) => Some(color),
            PieceType::Queen(color) => Some(color),
            PieceType::Pawn(color) => Some(color),
            PieceType::Bishop(color) => Some(color),
            PieceType::Rook(color) => Some(color),
            PieceType::Empty => None,
        }
    }

    pub fn is_empty_or_not_same_color(&self, dst: &Position, board: &Board) -> bool {
        let dst = board.get_piece_at_position(dst);
        match dst.piece_type {
            PieceType::Empty => true,
            _ => self.color().expect("src not empty") != dst.color().expect("dst not empty"),
        }
    }
}

// opponent AI
// random move
// pick random index
// if piece at index is valid, move it
// stockfish?
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: i8,
    pub col: i8,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Position {
    pub fn new(row: i8, col: i8) -> Position {
        Position { row, col }
    }

    pub fn increment_row(&mut self) {
        self.row += 1;
    }
    pub fn increment_col(&mut self) {
        self.col += 1;
    }
    pub fn decrement_row(&mut self) {
        self.row -= 1;
    }
    pub fn decrement_col(&mut self) {
        self.col -= 1;
    }

    pub fn from_str(s: &str) -> Position {
        let col = match s.chars().nth(0).unwrap() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => 0,
        };
        let row = 8 - s.chars().nth(1).unwrap().to_digit(10).unwrap() as i8;
        Position { row, col }
    }

    pub fn is_valid(&self) -> bool {
        self.row < 8 && self.col < 8 && self.row >= 0 && self.col >= 0
    }
}

impl Position {
    pub fn as_str(&self) -> String {
        let col = match self.col {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => "invalid",
        };
        let row = 8 - self.row;
        format!("{}{}", col, row)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Move {
    pub fn is_movement_valid(&self, from_pos: &Position) -> bool {
        todo!();
    }
}
