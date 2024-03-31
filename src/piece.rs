// struct Move
// has start?
// has end position
// has piece type
// has move type (capture, check, checkmate, etc)
#[derive(Debug)]
pub struct Move {
    start: Option<Position>,
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
    pub row: u8,
    pub col: u8,
}

#[derive(Debug)]
pub enum Color {
    White,
    Black,
}
