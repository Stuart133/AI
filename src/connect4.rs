use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Space {
    White,
    Black,
    Empty,
}

impl Default for Space {
    fn default() -> Self {
        Space::Empty
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Space::White => write!(f, "x"),
            Space::Black => write!(f, "o"),
            Space::Empty => write!(f, " "),
        }
    }
}

#[derive(Clone)]
struct Row([Space; 7]);

impl Default for Row {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for space in self.0.iter() {
            match write!(f, "{} ", space) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct Game {
    board: [Row; 6],
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: <[Row; 6]>::default(),
        }
    }

    pub fn add_piece(&self, column: usize, color: Space) -> Self {
        let mut new_board = self.clone();

        for i in 0..new_board.board.len() {
            match new_board.board[i].0[column] {
                Space::Empty => {} // Space is empty, keep going
                _ => {
                  new_board.board[i - 1].0[column] = color;
                  return new_board
                },
            }
        }

        // Fill the bottom space
        new_board.board[new_board.board.len()-1].0[column] = color;
        new_board
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write the board header
        let mut res = write!(f, "  0 1 2 3 4 5 6\n");

        for (i, line) in self.board.iter().enumerate() {
            res = res.and_then(|_| write!(f, "{} {}\n", i, line));
        }

        res
    }
}
