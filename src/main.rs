use std::path::Path;

pub mod algrebra;
pub mod connect4;
pub mod csp;
pub mod game;
pub mod graph;
pub mod nqueens;
pub mod nqueens_csp;
pub mod senate;
pub mod soduku;

fn main() {
    let data = senate::parse(Path::new("data/S109.ord"));
    println!("{:?}", data);
}
