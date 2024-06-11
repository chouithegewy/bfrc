#![allow(dead_code)]
#![allow(unused_variables)]
// struct Move
// has start?
// has end position
// has piece type
// has move type (capture, check, checkmate, etc)
#[derive(Debug)]
pub struct Move {
    pub start_pos: Option<String>,
    pub end_pos: Position,
    pub piece_type: PieceType,
    pub captures: bool,
    pub move_type: MoveType,
    pub check: bool,
}

// enum PieceType
// King, Queen, Rook, Bishop, Knight, Pawn, Empty
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub position: Position,
    pub move_history: Vec<Move>,
    pub move_set: Vec<Move>,
}

// opponent AI
// random move
// pick random index
// if piece at index is valid, move it
// stockfish?
#[derive(Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Position {
    pub fn new(row: usize, col: usize) -> Position {
        Position { row, col }
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
        let row = 8 - s.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
        Position { row, col }
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

#[derive(Debug)]
pub enum Color {
    White,
    Black,
}
