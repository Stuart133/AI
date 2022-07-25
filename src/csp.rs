use std::{
    collections::{HashMap, HashSet},
    hash::Hash, thread::sleep, time::Duration,
};

#[derive(Debug, Clone)]
pub struct Variable<T: Eq + Hash + Clone> {
    pub name: String,
    pub value: Option<T>,
    domain: HashSet<T>,
}

impl<T: Eq + Hash + Clone> Variable<T> {
    pub fn new(name: String, domain: HashSet<T>, value: Option<T>) -> Self {
        Variable {
            name,
            value,
            domain,
        }
    }

    pub fn assign(&mut self, value: &T) {
        self.value = self.domain.take(value);
        self.domain.clear();
    }

    pub fn is_assigned(&self) -> bool {
        self.value != None
    }
}

#[derive(Clone)]
pub struct BinaryConstraint<T> {
    left: usize,
    right: usize,
    check: fn(&T, &T) -> bool,
}

impl<T> BinaryConstraint<T> {
    pub fn new(left: usize, right: usize, check: fn(&T, &T) -> bool) -> Self {
        BinaryConstraint { left, right, check }
    }
}

/// A generic constraint solver, with variables of type T
#[derive(Clone)]
pub struct ConstraintSolver<T: Eq + Hash + Clone> {
    variables: Vec<Variable<T>>,
    constraints: HashMap<usize, Vec<BinaryConstraint<T>>>,
}

impl<T: Eq + Hash + Clone> ConstraintSolver<T> {
    pub fn new(
        variables: Vec<Variable<T>>,
        constraints: HashMap<usize, Vec<BinaryConstraint<T>>>,
    ) -> Self {
        ConstraintSolver {
            variables,
            constraints,
        }
    }

    pub fn solve(self, finished: fn(&Vec<Variable<T>>) -> bool) -> Vec<Variable<T>> {
        for (i, variable) in self.variables.iter().enumerate() {
            for value in variable.domain.iter() {
                // sleep(Duration::from_millis(1000));
                let mut new_csp = self.clone();
                new_csp.variables[i].assign(value);

                // Check constraints
                if !new_csp.check_constrains(i) {
                    continue;
                }

                // Check if we're finished
                if finished(&new_csp.variables) {
                    return new_csp.variables;
                }

                // Continue DFS
                let solved = new_csp.solve(finished);
                if solved.len() != 0 {
                    return solved;
                }
            }
        }

        vec![]
    }

    fn check_constrains(&self, last_set: usize) -> bool {
        let mut valid = true;

        for constraint in self.constraints.get(&last_set).unwrap() {
            if self.variables[constraint.right].value == None {
                continue;
            }

            valid = valid
                && (constraint.check)(
                    self.variables[constraint.left].value.as_ref().unwrap(),
                    self.variables[constraint.right].value.as_ref().unwrap(),
                );
        }

        valid
    }
}
