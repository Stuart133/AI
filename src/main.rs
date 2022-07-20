use crate::{connect4::*, game::{minimax, MinimaxGame}};

pub mod algrebra;
pub mod connect4;
pub mod game;
pub mod graph;

fn main() {
    let mut game = Game::new();
    game = game.add_piece(3);

    while !game.has_finished() {
        let m = minimax(game.clone(), 2);
        game = game.add_piece(m);

        println!("{}", game);
    }
}
