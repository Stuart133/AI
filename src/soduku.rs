use std::collections::{HashSet, HashMap};

use crate::csp::{Variable, BinaryConstraint};

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

    for (j, row) in grid.iter().enumerate() {
        for (i, item) in row.iter().enumerate() {
            match item {
                Some(value) => variables.push(Variable::new(format!("{} {}", i, j), HashSet::new(), Some(*value))),
                None => variables.push(Variable::new(format!("{} {}", i, j), domain.clone(), None)),
            }
        }
    }

    let mut constraints = HashMap::<(usize, usize), BinaryConstraint<i32>>::new();
    for (j1, row) in grid.iter().enumerate() {
      for (i1, item1) in row.iter().enumerate() {
        for (j2, row) in grid.iter().enumerate() {
          for (i2, item2) in row.iter().enumerate() {
            if item1 != item2 && (i1 == i2 || j1 == j2 || get_box(i1, j1) == get_box(i1, j2)) {
              constraints.insert((get_index(i1, j1), get_index(i2, j2)), BinaryConstraint::new(check));
            }
          }
        }
      }
    }
}

fn get_index(x: usize, y: usize) -> usize {
  x + (y * 9)
}

fn get_box(x: usize, y: usize) -> usize {
  x + (y * 3)
}

fn check(left: &i32, right: &i32) -> bool {
  left != right
}
