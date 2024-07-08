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
    while true {
        let userinput = parse_move::read_user_input();
        dbg!(&userinput);
        match parse_move::parse_user_input(userinput) {
            Some(usermove) => {
                dbg!(&usermove);
                let dummypiece = piece::Piece::new(
                    usermove.piece_type,
                    usermove.end_pos,
                );
                game.board.set_piece(dummypiece);
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
