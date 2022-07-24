use crate::nqueens::solve;

pub mod algrebra;
pub mod connect4;
pub mod csp;
pub mod game;
pub mod graph;
pub mod nqueens;
pub mod soduku;

fn main() {
    let solutions = solve::<14>();

    println!("{}", solutions.len());
}
