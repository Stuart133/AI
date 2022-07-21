use std::fmt::Display;

pub fn solve<const N: usize>() -> Vec<Queens<N>> {
  let queen = Queens::<N>::new();
  solve_internal(N - 1, queen)
}

fn solve_internal<const N: usize>(row: usize, root: Queens<N>) -> Vec<Queens<N>> {
  let mut solutions = vec![];

  if row == 0 {
    return root.get_safe_moves(row);
  }

  for next in root.get_safe_moves(row) {
    solutions.append(&mut solve_internal(row - 1, next));
  }

  solutions
}

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

    fn get_safe_moves(&self, y: usize) -> Vec<Queens<N>> {
      let mut queens = vec![];

      for x in 0..N {
        if self.safe_place(x, y) {
          queens.push(self.place_queen(x, y).expect("oops"));
        }
      }

      queens
    }

    fn safe_place(&self, x: usize, y: usize) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if i == 0 && j == 0 {
                    continue;
                }

                let (mut x0, mut y0) = (x, y);

                while x0 < N && y0 < N {
                    if self.board[y0][x0] {
                        return false;
                    }

                    if i == 2 {
                        x0 = x0.wrapping_sub(1);
                    } else {
                        x0 += i;
                    }

                    if j == 2 {
                        y0 = y0.wrapping_sub(1);
                    } else {
                        y0 += j;
                    }
                }
            }
        }

        true
    }

    pub fn place_queen(&self, x: usize, y: usize) -> Result<Queens<N>, String> {
        if x >= N || y >= N {
            return Err("Coordinates out of range".to_string());
        }

        let mut new_board = self.clone();
        new_board.board[y][x] = true;

        Ok(new_board)
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

#[cfg(test)]
mod tests {
    use super::Queens;

    #[test]
    pub fn safe_place() {
        let queens = Queens::<4>::new()
            .place_queen(0, 0)
            .unwrap()
            .place_queen(3, 2)
            .unwrap();

        assert!(!queens.safe_place(0, 0));
        assert!(!queens.safe_place(1, 0));
        assert!(!queens.safe_place(0, 1));
        assert!(!queens.safe_place(3, 3));
        assert!(!queens.safe_place(2, 1));

        assert!(queens.safe_place(1, 3));
    }

    #[test]
    pub fn get_safe_moves() {
        let queens = Queens::<4>::new()
            .place_queen(0, 0)
            .unwrap()
            .place_queen(3, 1)
            .unwrap();

        let moves = queens.get_safe_moves(2);

        assert_eq!(moves.len(), 1);
        assert!(moves[0].board[2][1]);

        let moves = queens.get_safe_moves(3);
        assert_eq!(moves.len(), 1);
        assert!(moves[0].board[3][2]);
    }
}
