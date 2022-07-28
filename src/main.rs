use std::path::Path;

use senate::{crosscheck, evaluate, hamming_distance};

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
    let data = senate::parse(Path::new("data/S110.ord"));

    let (g1, g2) = crosscheck(data);
    println!("{}", evaluate(hamming_distance, 1, &g1, &g2));
}
