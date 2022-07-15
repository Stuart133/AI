use crate::connect4::*;

mod algrebra;
pub mod connect4;
pub mod graph;

fn main() {
    let mut game = Game::new();

    game = game
        .add_piece(0, Space::White)
        .add_piece(0, Space::Black)
        .add_piece(0, Space::White)
        .add_piece(0, Space::Black)
        .add_piece(6, Space::White)
        .add_piece(6, Space::White)
        .add_piece(6, Space::Black);

    println!("{}", game);
}
