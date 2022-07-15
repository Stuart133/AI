use crate::connect4::*;

mod algrebra;
pub mod connect4;
pub mod graph;

fn main() {
    let game = Game::new();

    println!("{}", game);
}
