mod board;
mod game;
mod piece;
mod parse_move;

fn main() {
    let mut game = game::Game {
        board: board::Board::new(),
    };
    game.board.print();
    let piece = game.board.get_piece_at_index(0, 0);
    dbg!(piece);
    dbg!(piece.position.as_str());
    let usermove = parse_move::read_user_input();
    dbg!(&usermove);
    dbg!(parse_move::parse_user_input(usermove));
}
