use std::{fmt::Display, ops::{Index, IndexMut}};

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "x"),
            Color::Black => write!(f, "o"),
        }
    }
}

#[derive(Clone)]
struct Row([Option<Color>; WIDTH]);

impl Default for Row {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Index<usize> for Row {
    type Output = Option<Color>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Row {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for space in self.0 {
            let res = match space {
                Some(s) => write!(f, "{} ", s),
                None => write!(f, "  "),
            };

            match res {
              Err(e) => return Err(e),
              _ => {},
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Game {
    board: [Row; HEIGHT],
    current_player: Color,
    last_placement: (usize, usize),
}

impl<'a> Index<(usize, usize)> for Game {
    type Output = Option<Color>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 >= self.board.len() || index.1 >= self.board[index.0].0.len() {
            &None
        } else {
            &self.board[index.0][index.1]
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: <[Row; 6]>::default(),
            current_player: Color::White,
            last_placement: (0, 0),
        }
    }

    pub fn add_piece(&self, column: usize) -> Self {
        let mut new_board = self.clone();
        new_board.current_player = match self.current_player {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        for i in 0..new_board.board.len() {
            match new_board.board[i][column] {
                None => {} // Space is empty, keep going
                Some(_) => {
                    new_board.board[i - 1][column] = Some(self.current_player);
                    return new_board;
                }
            }
        }

        // Fill the bottom space
        new_board.board[new_board.board.len() - 1][column] = Some(self.current_player);
        new_board
    }

    /// Returns true if the current player has won
    pub fn has_won(&self) -> bool {
        // We only need to check chains from the last space placed
        false
    }

    fn get_longest_chain(&self, x: usize, y: usize) -> usize {
        self.board[x][y];

        0
    }

    fn get_longest_vector(&self, x: usize, y: usize, direction: (usize, usize)) -> usize {
        let color = self.board[x][y].expect("no token in space");

        let mut count = 1;
        while let Some(inner) = self.board[x+(direction.0 * count)][y+(direction.1 * count)] {
            if inner != color {
                break
            }    
            
            count += 1;
        }

        count - 1
    }

    // pub fn evaluate(&self) -> u64 {

    // }
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
