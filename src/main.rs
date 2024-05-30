mod board;
mod game;
mod piece;
mod parse_move;

fn main() {
    let mut game = game::Game {
        board: board::Board::new(),
    };
    game.board.print();
    let usermove = parse_move::read_user_input();
    dbg!(usermove);
}
