use std::{fmt::Display, ops::{Index, IndexMut}, cmp::max};

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
        if index.1 >= self.board.len() || index.0 >= self.board[index.1].0.len() {
            &None
        } else {
            &self.board[index.1][index.0]
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

    /// Get the longest chain of tokens matching the color in (x, y)
    fn get_longest_chain(&self, x: usize, y: usize) -> usize {
        let ud = self.get_longest_vector(x, y, Direction::Up) + self.get_longest_vector(x, y, Direction::Down) + 1;
        let lr = self.get_longest_vector(x, y, Direction::Left) + self.get_longest_vector(x, y, Direction::Right) + 1;
        let urdl = self.get_longest_vector(x, y, Direction::UpRight) + self.get_longest_vector(x, y, Direction::DownLeft) + 1;
        let uldr = self.get_longest_vector(x, y, Direction::UpLeft) + self.get_longest_vector(x, y, Direction::DownRight) + 1;

        max(ud, max(lr, max(urdl, uldr)))
    }

    /// Return the longest chain of tokens matching the color in (x, y) in the specified direction
    fn get_longest_vector(&self, x: usize, y: usize, direction: Direction) -> usize {
        let color = self[(x, y)].expect("no token in space");

        let mut count = 0;
        let (mut new_x, mut new_y) = direction.calculate_new_indices(x, y);

        while let Some(inner) = self[(new_x, new_y)] {
            if inner != color {
                break
            }

            let new = direction.calculate_new_indices(new_x, new_y);
            new_x = new.0;
            new_y = new.1;
            
            count += 1;
        }

        count
    }

    // pub fn evaluate(&self) -> u64 {

    // }
}

impl Direction {
    fn calculate_new_indices(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::Up => (x, y.wrapping_sub(1)),
            Direction::UpLeft => (x.wrapping_sub(1), y.wrapping_sub(1)),
            Direction::Left => (x.wrapping_sub(1), y),
            Direction::DownLeft => (x.wrapping_sub(1), y),
            Direction::Down => (x, y + 1),
            Direction::DownRight => (x + 1, y + 1),
            Direction::Right => (x + 1, y),
            Direction::UpRight => (x + 1, y.wrapping_sub(1)),
        }
    }
}

enum Direction {
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
    Right,
    UpRight,
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

#[cfg(test)]
mod tests {
    use crate::connect4::Direction;

    use super::Game;

    #[test]
    pub fn get_longest_vector() {
        let game = Game::new();

        let game = game.add_piece(0)
        .add_piece(1)
        .add_piece(0)
        .add_piece(1)
        .add_piece(0)
        .add_piece(1)
        .add_piece(0);

        let longest = game.get_longest_vector(0, 5, Direction::Up);
        assert_eq!(3, longest);

        let longest = game.get_longest_vector(1, 3, Direction::Down);
        assert_eq!(2, longest);
    }

    #[test]
    pub fn get_longest_chain() {
        let game = Game::new();

        let game = game.add_piece(0)
        .add_piece(1)
        .add_piece(0)
        .add_piece(1)
        .add_piece(1)
        .add_piece(0)
        .add_piece(2)
        .add_piece(4)
        .add_piece(2)
        .add_piece(4)
        .add_piece(3)
        .add_piece(2)
        .add_piece(3)
        .add_piece(4)
        .add_piece(3)
        .add_piece(3)
        .add_piece(2)
        .add_piece(1)
        .add_piece(4)
        .add_piece(0);


        let longest = game.get_longest_chain(3, 3);
        assert_eq!(3, longest);

        let longest = game.get_longest_chain(1, 2);
        assert_eq!(2, longest);
    }
}