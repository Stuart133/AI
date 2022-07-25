use std::collections::{HashMap, HashSet};

use crate::csp::{BinaryConstraint, ConstraintSolver, Variable};

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
                Some(value) => variables.push(Variable::new(
                    format!("{} {}", i, j),
                    HashSet::new(),
                    Some(*value),
                )),
                None => variables.push(Variable::new(format!("{} {}", i, j), domain.clone(), None)),
            }
        }
    }

    let mut constraints = HashMap::<usize, Vec<BinaryConstraint<i32>>>::new();
    for (j1, row) in grid.iter().enumerate() {
        for (i1, _) in row.iter().enumerate() {
            for (j2, row) in grid.iter().enumerate() {
                for (i2, _) in row.iter().enumerate() {
                    if !(i1 == i2 && j1 == j2)
                        && (i1 == i2 || j1 == j2 || (get_box(i1, j1) == get_box(i2, j2)))
                    {
                        let current = constraints.get_mut(&get_index(i1, j1));
                        match current {
                            Some(c) => c.push(BinaryConstraint::new(
                                get_index(i1, j1),
                                get_index(i2, j2),
                                check,
                            )),
                            None => {
                                constraints.insert(
                                    get_index(i1, j1),
                                    vec![BinaryConstraint::new(
                                        get_index(i1, j1),
                                        get_index(i2, j2),
                                        check,
                                    )],
                                );
                            }
                        };
                    }
                }
            }
        }
    }

    let csp = ConstraintSolver::new(variables, constraints, None);
    let solution = csp.solve(finished);

    for var in solution {
        println!("{:?}", var.value);
    }
}

fn get_index(x: usize, y: usize) -> usize {
    x + (y * 9)
}

fn get_box(x: usize, y: usize) -> usize {
    x / 3 + (y / 3) * 3
}

fn check(left: &i32, right: &i32) -> bool {
    left != right
}

fn finished(variables: &Vec<Variable<i32>>) -> bool {
    for variable in variables {
        match variable.value {
            Some(_) => {}
            None => return false,
        }
    }

    true
}
