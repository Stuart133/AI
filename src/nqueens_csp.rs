use std::collections::{HashMap, HashSet};

use crate::csp::{BinaryConstraint, ConstraintSolver, Variable};

pub fn solve_queens() {
    let mut variables = vec![];
    let domain: HashSet<bool> = vec![true, false].into_iter().collect();

    for _ in 0..16 {
        variables.push(Variable::new("".to_string(), domain.clone(), None));
    }

    let mut constraints = HashMap::<usize, Vec<BinaryConstraint<bool>>>::new();
    for (i, _) in variables.iter().enumerate() {
        for (j, _) in variables.iter().enumerate() {
            let i_xy = get_xy(i);
            let j_xy = get_xy(j);

            if i != j
                && (i_xy.0 == j_xy.0
                    || i_xy.1 == j_xy.1
                    || on_diagonal(i_xy.0, j_xy.0, i_xy.1, j_xy.1))
            {
                let current = constraints.get_mut(&i);
                match current {
                    Some(c) => c.push(BinaryConstraint::new(i, j, check)),
                    None => {
                        constraints.insert(i, vec![BinaryConstraint::new(i, j, check)]);
                    }
                }
            }
        }
    }

    let csp = ConstraintSolver::new(variables, constraints);
    let solution = csp.solve(finished);

    for var in solution {
        println!("{:?}", var.value);
    }
}

fn on_diagonal(x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    let diff_x = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let diff_y = if y1 > y2 { y1 - y2 } else { y2 - y1 };

    diff_x == diff_y
}

fn get_xy(index: usize) -> (usize, usize) {
    (index % 4, index / 4)
}

fn check(left: &bool, right: &bool) -> bool {
    !(*left && *right)
}

// Check that all variables are assigned and there are exactly n queens
fn finished(variables: &Vec<Variable<bool>>) -> bool {
    let mut queens = 0;

    for variable in variables {
        match variable.value {
            Some(q) => {
                if q {
                    queens += 1;
                }
            }
            None => return false,
        }
    }

    queens == 4
}
