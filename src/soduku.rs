use std::collections::HashSet;

use crate::csp::Variable;

pub fn solve_soduku() {
    #[rustfmt::skip]
  let grid = [[None, None, None, None, Some(6), None, None, Some(3), None],
    [Some(2), None, Some(7), Some(3), Some(8), None, None, Some(4), None],
    [Some(6), None, None, None, None, None, None, Some(2), None],
    [Some(3), None, None, Some(1), Some(4), Some(5), Some(2), None, None],
    [Some(5), Some(6), Some(1), Some(2), Some(9), None, None, Some(7), None],
    [None, None, Some(9), None, Some(7), None, None, None, Some(1)],
    [Some(8), None, Some(2), Some(5), Some(1), None, Some(6), None, Some(4)],
    [None, Some(4), None, Some(9), Some(3), Some(6), Some(8), None, None],
    [None, Some(9), None, None, None, Some(4), Some(3), None, Some(7)]];

    let mut variables = vec![];
    let domain: HashSet<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();

    for row in grid.iter() {
        for item in row.iter() {
            match item {
                Some(value) => variables.push(Variable::new(HashSet::new(), Some(*value))),
                None => variables.push(Variable::new(domain.clone(), None)),
            }
        }
    }
}
