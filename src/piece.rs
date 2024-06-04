#![allow(dead_code)]
#![allow(unused_variables)]
// struct Move
// has start?
// has end position
// has piece type
// has move type (capture, check, checkmate, etc)
#[derive(Debug)]
pub struct Move {
    start: Position,
    end: Position,
    piece_type: PieceType,
    move_type: MoveType,
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
// Capture, Check, Checkmate, EnPassant, Castling, Promotion, Normal
#[derive(Debug)]
pub enum MoveType {
    Capture,
    Check,
    Checkmate,
    EnPassant,
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
