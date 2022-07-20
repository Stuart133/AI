use crate::{connect4::*, game::minimax};

pub mod algrebra;
pub mod connect4;
pub mod game;
pub mod graph;

fn main() {
    let mut game = Game::new();

    let m = minimax(game.clone(), 5);

    println!("{}", m);
}
