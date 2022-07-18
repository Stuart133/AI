use crate::connect4::*;

mod algrebra;
pub mod connect4;
pub mod graph;

fn main() {
    let mut game = Game::new();

    game = game
        .add_piece(0)
        .add_piece(1)
        .add_piece(0)
        .add_piece(1)
        .add_piece(0)
        .add_piece(1)
        .add_piece(0);

    println!("{}", game);
}
