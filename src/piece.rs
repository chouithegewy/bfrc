#![allow(dead_code)]
#![allow(unused_variables)]
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
#[derive(Debug, Clone, Copy)]
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
            PieceType::King(color) => {},
            PieceType::Queen(color) => {},
            PieceType::Bishop(color) => {},
            PieceType::Rook(color) => {},
            PieceType::Pawn(color) => {},
            PieceType::Knight(color) => {},
            _ => {},
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
    // a move is legal if it is valid^ and if it satifies the following:
    // - the player's king cannot be in check after the move (a piece that is 'pinned' cannot move, the king
    // itself cannot be placed into check, including through castling)
    // - the player's piece cannot move to a square already occupied by his own piece
    // - each piece must follow its rules for movement and cannot move through another occupied
    // piece (a knight can "hop", though)
    // the method of determining legal moves is as follows:
    // 1) check if valid
    // 2) check if movement is correct for given piece and own piece does not occupy the square
    // 3) make move
    // 4) populate new board state, if the player's king is in check after making the move, then it
    //    is an illegal move and revert the state
    pub fn is_legal(&self, old_position: &Position) -> bool {
        !(!&self.is_valid()) //lol wat
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

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Move {
    pub fn is_movement_valid(&self, from_pos: &Position) -> bool {
        todo!();
    }

}
