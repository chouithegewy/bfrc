use std::collections::HashSet;

use crate::{piece::Position, piece_movement::generate_move_set};

mod board;
mod game;
mod parse_move;
mod piece;
mod piece_movement;

fn main() {
    let mut game = game::Game {
        board: board::Board::new(),
    };
    game.board.print();
    loop {
        let userinput = parse_move::read_user_input();
        match parse_move::parse_user_input(userinput) {
            Some(usermove) => {
                let dummypiece = piece::Piece::new(usermove.piece_type, usermove.end_pos);
                let pieces_of_type = game.board.get_pieces_of_type(dummypiece.piece_type);
                let mut count_of_pieces_that_can_move_to_that_location = 0; // 1 == ok
                let mut originating_square = Position::new(-1, -1);
                for p in pieces_of_type {
                    let dummy_move_set_pos_str = generate_move_set(p, &game.board)
                        .iter()
                        .map(|p| p.end_pos.as_str())
                        .collect::<Vec<_>>();
                    let move_set: HashSet<&String> = HashSet::from_iter(&dummy_move_set_pos_str);
                    if move_set.contains(&dummypiece.position.as_str()) {
                        count_of_pieces_that_can_move_to_that_location += 1;
                        originating_square = p.position;
                    }
                }
                if count_of_pieces_that_can_move_to_that_location == 1 {
                    game.board.set_piece(dummypiece);
                    game.board.set_square_empty(&originating_square);
                } else {
                    println!(
                        "Invalid move. Count of pieces that can move to that location: {}",
                        count_of_pieces_that_can_move_to_that_location
                    );
                    continue;
                }
                //see if any piece can move to that location
                //game.board.get_piece_at(that).move_piece(to);
                //check if move is valid
                //make move
                game.board.print();
            }
            None => {
                println!("Invalid move");
            }
        }
    }
}
