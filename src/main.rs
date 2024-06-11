mod board;
mod game;
mod piece;
mod parse_move;

fn main() {
    let mut game = game::Game {
        board: board::Board::new(),
    };
    game.board.print();
    let userinput = parse_move::read_user_input();
    dbg!(&userinput);
    dbg!(parse_move::parse_user_input(userinput));
}
