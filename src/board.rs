#![allow(dead_code)]
#![allow(unused_variables)]
use crate::{
    piece::{Color, Move, MoveType, Piece, PieceType, Position},
    piece_movement::generate_move_set,
};

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
    pub fn get_piece_at_position(&self, position: &Position) -> &Piece {
        &self.board[8 * position.row as usize + position.col as usize]
    }

    pub fn get_mut_piece_at_position(&mut self, position: &Position) -> &mut Piece {
        &mut self.board[8 * position.row as usize + position.col as usize]
    }

    pub fn set_piece_at_position(&mut self, piece: Piece, position: &Position) {
        self.board[8 * position.row as usize + position.col as usize] = piece;
    }

    pub fn get_piece_at_index(&self, index: usize) -> &Piece {
        &self.board[index]
    }

    pub fn get_pieces_of_type(&self, piece_type: PieceType) -> Vec<&Piece> {
        let mut pieces = vec![];
        for piece in self.board.iter() {
            if piece.piece_type == piece_type {
                pieces.push(piece);
            }
        }
        pieces
    }

    pub fn set_piece(&mut self, piece: Piece) {
        let row = piece.position.row as usize;
        let col = piece.position.col as usize;
        self.board[8 * row + col] = piece;
    }
    pub fn set_square_empty(&mut self, position: &Position) {
        let p = self.get_mut_piece_at_position(position);
        p.piece_type = PieceType::Empty;
    }

    pub fn get_black_pieces(&self) -> Vec<&Piece> {
        let mut black_pieces = vec![];
        for p in self.board.iter() {
            let black_piece = match p.piece_type {
                PieceType::King(color) => color == Color::Black,
                PieceType::Queen(color) => color == Color::Black,
                PieceType::Bishop(color) => color == Color::Black,
                PieceType::Rook(color) => color == Color::Black,
                PieceType::Pawn(color) => color == Color::Black,
                PieceType::Knight(color) => color == Color::Black,
                PieceType::Empty => false,
            };
            if black_piece == true {
                black_pieces.push(p);
            }
        }
        black_pieces
    }

    // generate a random move for black
    pub fn generate_random_legal_move_for_black(&mut self) -> Option<Move> {
        let black_pieces = self.get_black_pieces();
        for bp in black_pieces {
            let bm = Move {
                start_pos: Some(bp.position),
                end_pos: None,
                piece_type: bp.piece_type,
                captures: false,
                move_type: MoveType::Normal,
                check: false,
            }; // change
            let moves = generate_move_set(&bm, self, bp.color().expect("Failed to get color"));
            for m in moves {
                dbg!(&m);
                //let backup =
                //    self.get_piece_at_position(&m.end_pos.expect("Failed to unwrap end_pos"));
                //let piece = Piece::new(m.piece_type, m.end_pos.expect("Failed to unwrap end_pos"));
                //self.set_piece(piece.clone());
                //self.set_square_empty(&backup.position);
                //self.print();
                return Some(m);
            }
        }
        None
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
    //pub fn is_legal(&self, src: &Position, dst: &Position) -> bool {
    //    if src.is_valid() && dst.is_valid() {
    //        let piece = self.get_piece_at_position(src);
    //        let move_set = generate_move_set(piece, self);
    //    }
    //    return false;
    //}
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
