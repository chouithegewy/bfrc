use crate::piece::{Color, Move, MoveType, PieceType, Position};
use std::io;

// user input is shorthand algebraic chess notation
// Nf3, Qxa2, Kh8+, etc
// parse string into move rust type
// check if move is legal or not
// if legal, update board state
// if illegal, return error message
//
//

pub fn read_user_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

pub fn parse_user_input(input: String) -> Option<Move> {
    //-> Move {
    let regex = regex::Regex::new(r"(([Q|K|N|R|B])?([a-h]|[1-8])?)(x)?([a-h][1-8])([+])?").unwrap();
    let caps = regex.captures(&input);
    match caps {
        Some(caps) => {
            if caps.get(0).unwrap().len() != input.len() {
                println!("Invalid move");
                return None;
            } else {
                let usermove = Move {
                    start_pos: match &caps.get(1) {
                        Some(start_pos) => {
                            if start_pos.len() == 2 {
                                Some(Position::from_str(start_pos.as_str()))
                            } else {
                                None
                            }
                        }
                        None => None,
                    },
                    end_pos: Position::from_str(caps.get(5).unwrap().as_str()),
                    piece_type: match &caps.get(2) {
                        Some(piece) => match piece.as_str() {
                            "Q" => PieceType::Queen(Color::White),
                            "K" => PieceType::King(Color::White),
                            "N" => PieceType::Knight(Color::White),
                            "R" => PieceType::Rook(Color::White),
                            "B" => PieceType::Bishop(Color::White),
                            _ => panic!("Parse error: invalid piece type; expected Q, K, N, R, B"),
                        },
                        None => PieceType::Pawn(Color::White),
                    },
                    captures: match &caps.get(4) {
                        Some(move_type) => match move_type.as_str() {
                            "x" => true,
                            _ => panic!("Parse error: invalid move type; expected x"),
                        },
                        None => false,
                    },
                    move_type: MoveType::Normal,
                    check: match &caps.get(6) {
                        Some(move_type) => match move_type.as_str() {
                            "+" => true,
                            _ => panic!("Parse error: invalid move type; expected +"),
                        },
                        None => false,
                    },
                };
                Some(usermove)
            }
        }
        None => {
            println!("Invalid move");
            None
        }
    }
}
