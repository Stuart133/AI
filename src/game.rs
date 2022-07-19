use crate::connect4::Game;

// TODO: Use trait to make this generic
pub fn minimax(game: &Game, depth: usize) -> usize {
    game.get_moves()
      .map(|(new_move, game)| (new_move, -1 * minimax_value(&game, depth - 1)))
      .reduce(|acc, (new_move, value)| {
        if acc.1 > value {
          (new_move, value)
        } else {
          acc
        }
      });

    0
}

/// Returns the minimax value from the current node, searching as far as depth
fn minimax_value(game: &Game, depth: usize) -> i64 {
    if depth <= 0 || game.has_finished() {
        return game.evaluate();
    }

    game.get_moves()
        .map(|(_, game)| -1 * minimax_value(&game, depth - 1))
        .max()
        .expect("tried to expand game node with no more moves")
}
