use nqueens::Queens;

pub mod algrebra;
pub mod connect4;
pub mod game;
pub mod graph;
pub mod nqueens;

fn main() {
    let queens = Queens::<4>::new()
        .place_queen(0, 0).expect("oops");

    println!("{}", queens);
}
