use crate::nqueens::solve;

pub mod algrebra;
pub mod connect4;
pub mod game;
pub mod graph;
pub mod nqueens;

fn main() {
    let solutions = solve::<6>();

    for solution in solutions {
        println!("{}", solution);
    }
}
