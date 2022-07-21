use std::fmt::Display;

#[derive(Clone)]
pub struct Queens<const N: usize> {
    board: [[bool; N]; N],
}

impl<const N: usize> Queens<N> {
  pub fn new() -> Self {
    Queens {
      board: [[false; N]; N],
    }
  }

  pub fn place_queen(&self, x: usize, y: usize) -> Result<Queens<N>, String> {
    if x >= N || y >= N {
      return Err("Coordinates out of range".to_string());
    }

    let mut new_board = self.clone();
    new_board.board[y][x] = true;

    Ok(new_board);
  }
}

impl<const N: usize> Display for Queens<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = write!(f, "  ");
        for i in 0..N {
            res = res.and_then(|_| write!(f, "{} ", i));
        }

        for (y, row) in self.board.iter().enumerate() {
            res = res.and_then(|_| write!(f, "\n{} ", y));

            for x in row.iter() {
                if *x {
                    res = res.and_then(|_| write!(f, "Q  ",));
                } else {
                  res = res.and_then(|_| write!(f, "  "));
                }
            }
        }

        res
    }
}
