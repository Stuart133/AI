use std::cmp::max;

pub trait MinimaxGame<T: Iterator<Item = (usize, Self)>> {
    fn evaluate(&self, depth: usize) -> i64;
    fn has_finished(&self) -> bool;
    fn get_moves(self) -> T;
}

/// Returns the best move, searching as far as depth
pub fn minimax<T: MinimaxGame<I>, I: Iterator<Item = (usize, T)>>(game: T, depth: usize) -> usize {
    let new_move = game
        .get_moves()
        .map(|(new_move, game)| (new_move, -1 * minimax_value(game, depth - 1)))
        .reduce(|acc, (new_move, value)| {
            if acc.1 < value {
                (new_move, value)
            } else {
                acc
            }
        })
        .expect("could not find move");

    new_move.0
}

/// Returns the minimax value from the current node, searching as far as depth
fn minimax_value<T: MinimaxGame<I>, I: Iterator<Item = (usize, T)>>(game: T, depth: usize) -> i64 {
    if depth <= 0 || game.has_finished() {
        return game.evaluate(depth);
    }

    game.get_moves()
        .map(|(_, game)| -1 * minimax_value(game, depth - 1))
        .max()
        .expect("tried to expand game node with no more moves")
}

/// Returns the best move using alpha beta pruning, searching as far as depth
pub fn alpha_beta<T: MinimaxGame<I>, I: Iterator<Item = (usize, T)>>(game: T, depth: usize) -> usize {
    let new_move = game
        .get_moves()
        .map(|(new_move, game)| (new_move, -1 * alpha_beta_value(game, depth - 1, i64::min_value() + 1, i64::max_value())))
        .reduce(|acc, (new_move, value)| {
            if acc.1 < value {
                (new_move, value)
            } else {
                acc
            }
        })
        .expect("could not find move");

    new_move.0
}

/// Returns the minimax value from the current node using alpha beta pruning, searching as far as depth
fn alpha_beta_value<T: MinimaxGame<I>, I: Iterator<Item = (usize, T)>>(game: T, depth: usize, mut alpha: i64, beta: i64) -> i64 {
    if depth <= 0 || game.has_finished() {
        return game.evaluate(depth);
    }

    for (_, game) in game.get_moves() {
        // Swap and negate alpha & beta so next level maximises correctly
        let val = -1 * alpha_beta_value(game, depth, -beta, -alpha);

        alpha = max(alpha, val);
        if alpha > beta {
            break
        }
    }

    alpha
}

#[cfg(test)]
mod tests {
    use crate::game::{alpha_beta, alpha_beta_value};

    use super::*;

    #[derive(Clone)]
    struct Tree {
        links: Vec<Node>,
        root: usize,
    }

    #[derive(Clone)]
    enum Node {
        Leaf(i64),
        Node(usize, usize),
    }

    struct TreeIter {
        tree: Tree,
        left: usize,
        right: usize,
        index: usize,
    }

    impl<'a> Iterator for TreeIter {
        type Item = (usize, Tree);

        fn next(&mut self) -> Option<Self::Item> {
            if self.index == 2 {
                return None;
            }

            if self.index == 0 {
                self.tree.root = self.left;
            } else {
                self.tree.root = self.right;
            };

            self.index += 1;

            Some((self.index - 1, self.tree.clone()))
        }
    }

    impl MinimaxGame<TreeIter> for Tree {
        fn evaluate(&self, _: usize) -> i64 {
            match self.links[self.root] {
                Node::Leaf(val) => val,
                Node::Node(_, _) => panic!("shouldn't get here"),
            }
        }

        fn has_finished(&self) -> bool {
            match self.links[self.root] {
                Node::Leaf(_) => true,
                Node::Node(_, _) => false,
            }
        }

        fn get_moves(self) -> TreeIter {
            match self.links[self.root] {
                Node::Leaf(_) => panic!("shouldn't get here"),
                Node::Node(left, right) => TreeIter {
                    tree: self.clone(),
                    left,
                    right,
                    index: 0,
                },
            }
        }
    }

    struct TestData {
        tree: Tree,
        minimax_value: i64,
        next_move: usize,
    }

    fn get_test_data() -> Vec<TestData> {
        vec![
            TestData {
                tree: Tree {
                    links: vec![
                        Node::Node(1, 2),
                        Node::Node(3, 4),
                        Node::Node(5, 6),
                        Node::Node(7, 8),
                        Node::Node(9, 10),
                        Node::Node(11, 12),
                        Node::Node(13, 14),
                        Node::Leaf(-2),
                        Node::Leaf(-2),
                        Node::Leaf(0),
                        Node::Leaf(-4),
                        Node::Leaf(-6),
                        Node::Leaf(-8),
                        Node::Leaf(-4),
                        Node::Leaf(-6),
                    ],
                    root: 0,
                },
                minimax_value: 6,
                next_move: 1,
            },
            TestData {
                tree: Tree {
                    links: vec![
                        Node::Node(1, 2),
                        Node::Node(3, 4),
                        Node::Node(5, 6),
                        Node::Node(7, 8),
                        Node::Node(9, 10),
                        Node::Node(11, 12),
                        Node::Node(13, 14),
                        Node::Leaf(-6),
                        Node::Leaf(-4),
                        Node::Leaf(-8),
                        Node::Leaf(-6),
                        Node::Leaf(-4),
                        Node::Leaf(0),
                        Node::Leaf(-2),
                        Node::Leaf(-2),
                    ],
                    root: 0,
                },
                minimax_value: 6,
                next_move: 0,
            },
            TestData {
                tree: Tree {
                    links: vec![
                        Node::Node(1, 2),
                        Node::Node(3, 4),
                        Node::Node(5, 6),
                        Node::Node(7, 8),
                        Node::Leaf(7),
                        Node::Node(9, 10),
                        Node::Node(11, 12),
                        Node::Leaf(-8),
                        Node::Leaf(-2),
                        Node::Node(13, 14),
                        Node::Leaf(-3),
                        Node::Leaf(-9),
                        Node::Node(15, 16),
                        Node::Leaf(4),
                        Node::Leaf(5),
                        Node::Leaf(10),
                        Node::Leaf(8),
                    ],
                    root: 0,
                },
                minimax_value: 7,
                next_move: 0,
            }
        ]
    }

    #[test]
    pub fn tree_minimax_value() {
        for data in get_test_data() {
            let value = minimax_value(data.tree, 10);

            assert_eq!(value, data.minimax_value);
        }
    }

    #[test]
    pub fn tree_minimax() {
      for data in get_test_data() {
        let next_move = minimax(data.tree, 10);
        
        assert_eq!(next_move, data.next_move);
      }
    }

    #[test]
    pub fn tree_alphabeta_value() {
        for data in get_test_data() {
            let value = alpha_beta_value(data.tree, 10, i64::min_value() + 1, i64::max_value());

            assert_eq!(value, data.minimax_value);
        }
    }

    #[test]
    pub fn tree_alphabeta() {
      for data in get_test_data() {
        let next_move = alpha_beta(data.tree, 10);
        
        assert_eq!(next_move, data.next_move);
      }
    }
}
