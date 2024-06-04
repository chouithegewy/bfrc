use super::piece::*;
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

pub fn parse_user_input(input: String) {
    //-> Move {
    let regex = regex::Regex::new(r"([Q|K|N|R|B])?([a-h]|[1-8])?(x)?([a-h][1-8])([+])?").unwrap();
    let caps = regex.captures(&input);
    match caps {
        Some(caps) => {dbg!(&caps);}
        None => {
            println!("Invalid move");
        }
    };
}
