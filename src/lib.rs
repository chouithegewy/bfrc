// struct Board
//
// has flat representation of the board (row major order)
// width * row_index + col_index = index
// 00 01 02 03 04 05 06 07 10 11 12 13 14 15 16 17
// row 0 col 0 == (8 * 0 + 0) = 0
// row 0 col 7 == (8 * 0 + 7) = 7
// row 1 col 0 == (8 * 1 + 0) = 8
// row 7 col 7 == (8 * 7 + 7) = 63
// shared linear space wasm scalar
// get pgn string of board state

pub struct Board {
    board: [Piece; 64],
}

impl Board {
    fn new(self) -> Board {
        Board {
            board: [
                Piece {
                    piece_type: PieceType::Rook(Color::Black),
                    position: Position { row: 0, col: 0 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Knight(Color::Black),
                    position: Position { row: 0, col: 1 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Bishop(Color::Black),
                    position: Position { row: 0, col: 2 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Queen(Color::Black),
                    position: Position { row: 0, col: 3 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::King(Color::Black),
                    position: Position { row: 0, col: 4 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Bishop(Color::Black),
                    position: Position { row: 0, col: 5 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Knight(Color::Black),
                    position: Position { row: 0, col: 6 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Rook(Color::Black),
                    position: Position { row: 0, col: 7 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::Black),
                    position: Position { row: 1, col: 0 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::Black),
                    position: Position { row: 1, col: 1 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::Black),
                    position: Position { row: 1, col: 2 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::Black),
                    position: Position { row: 1, col: 3 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::Black),
                    position: Position { row: 1, col: 4 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::Black),
                    position: Position { row: 1, col: 5 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::White),
                    position: Position { row: 1, col: 6 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::White),
                    position: Position { row: 1, col: 7 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 0 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 1 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 2 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 3 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 4 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 5 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 6 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 7 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 0 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 1 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 2 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 3 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 4 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 5 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 6 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 2, col: 7 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 3, col: 0 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 3, col: 1 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 3, col: 2 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 3, col: 3 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 3, col: 4 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 3, col: 5 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 3, col: 6 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 3, col: 7 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 4, col: 0 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 4, col: 1 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 4, col: 2 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 4, col: 3 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 4, col: 4 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 4, col: 5 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 4, col: 6 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 4, col: 7 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 5, col: 0 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 5, col: 1 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 5, col: 2 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 5, col: 3 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 5, col: 4 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 5, col: 5 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 5, col: 6 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Empty,
                    position: Position { row: 5, col: 7 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::White),
                    position: Position { row: 6, col: 0 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::White),
                    position: Position { row: 6, col: 1 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::White),
                    position: Position { row: 6, col: 2 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::White),
                    position: Position { row: 6, col: 3 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::White),
                    position: Position { row: 6, col: 4 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::White),
                    position: Position { row: 6, col: 5 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::White),
                    position: Position { row: 6, col: 6 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::White),
                    position: Position { row: 6, col: 7 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Rook(Color::White),
                    position: Position { row: 7, col: 0 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Knight(Color::White),
                    position: Position { row: 7, col: 1 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Bishop(Color::White),
                    position: Position { row: 7, col: 2 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Queen(Color::White),
                    position: Position { row: 7, col: 3 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::King(Color::White),
                    position: Position { row: 7, col: 4 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Bishop(Color::White),
                    position: Position { row: 7, col: 5 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Knight(Color::White),
                    position: Position { row: 7, col: 6 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Rook(Color::White),
                    position: Position { row: 7, col: 7 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
            ],
        }
    }
}
// user input is shorthand algebraic chess notation
// Nf3, Qxa2, Kh8+, etc
// parse string into move rust type
// check if move is legal or not
// if legal, update board state
// if illegal, return error message
//
// struct Move
// has start?
// has end position
// has piece type
// has move type (capture, check, checkmate, etc)

pub struct Move {
    start: Option<Position>,
    end: Position,
    piece_type: PieceType,
    move_type: MoveType,
}

// enum PieceType
// King, Queen, Rook, Bishop, Knight, Pawn, Empty

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
//

pub struct Piece {
    piece_type: PieceType,
    position: Position,
    move_history: Vec<Move>,
    move_set: Vec<Move>,
}

//
//
// opponent AI
// random move
// pick random index
// if piece at index is valid, move it
// stockfish?
//
//

pub struct Position {
    row: u8,
    col: u8,
}

pub enum Color {
    White,
    Black,
}
