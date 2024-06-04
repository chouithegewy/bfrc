use crate::piece::{Color, Piece, PieceType, Position};

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
    pub fn get_piece_at_position(&self, position: Position) -> &Piece {
        &self.board[8 * position.row * position.col]
    }

    pub fn get_piece_at_index(&self, row: usize, col: usize) -> &Piece {
        &self.board[8 * row * col]
    }
}

impl Board {
    pub fn print(&self) {
        for (i, piece) in self.board.iter().enumerate() {
            if i % 8 == 0 {
                println!();
            }
            match piece.piece_type {
                PieceType::Empty => print!("_ "),
                _ => match piece.piece_type {
                    PieceType::King(Color::Black) => print!("k "),
                    PieceType::Queen(Color::Black) => print!("q "),
                    PieceType::Rook(Color::Black) => print!("r "),
                    PieceType::Bishop(Color::Black) => print!("b "),
                    PieceType::Knight(Color::Black) => print!("n "),
                    PieceType::Pawn(Color::Black) => print!("p "),
                    PieceType::King(Color::White) => print!("K "),
                    PieceType::Queen(Color::White) => print!("Q "),
                    PieceType::Rook(Color::White) => print!("R "),
                    PieceType::Bishop(Color::White) => print!("B "),
                    PieceType::Knight(Color::White) => print!("N "),
                    PieceType::Pawn(Color::White) => print!("P "),
                    _ => print!("?"),
                },
            }
        }
        println!();
    }
}

impl Board {
    pub fn new() -> Board {
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
                    piece_type: PieceType::Pawn(Color::Black),
                    position: Position { row: 1, col: 6 },
                    move_history: Vec::new(),
                    move_set: Vec::new(),
                },
                Piece {
                    piece_type: PieceType::Pawn(Color::Black),
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
