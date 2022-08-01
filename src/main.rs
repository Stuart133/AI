use std::path::Path;

use senate::DisorderTree;

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
    let bills = senate::parse_bills(Path::new("data/S110desc.csv"));

    let tree = DisorderTree::new(data.iter().map(|l| l).collect());
    tree.print(&bills);
}
