use std::fmt::Display;

#[derive(Clone, Copy)]
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

struct Row([Space; 7]);

impl Default for Row {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for space in self.0.iter() {
            match write!(f, "{}", space) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct Game {
    board: [Row; 6],
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: <[Row; 6]>::default(),
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write the board header
        let mut res = write!(f, "  0 1 2 3 4 5 6\n");

        for (i, line) in self.board.iter().enumerate() {
          res = res.and_then(|_| {
            write!(f, "{} {}\n", i, line)
          });
        }

        res
    }
}
