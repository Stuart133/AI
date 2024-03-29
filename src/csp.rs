use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

#[derive(Debug, Clone)]
pub struct Variable<T: Eq + Hash + Clone + Debug> {
    pub name: String,
    pub value: Option<T>,
    domain: HashSet<T>,
}

impl<T: Eq + Hash + Clone + Debug> Variable<T> {
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

#[derive(Clone)]
pub struct GlobalConstraint<T: Eq + Hash + Clone + Debug> {
    check: fn(&Vec<Variable<T>>) -> bool,
}

impl<T: Eq + Hash + Clone + Debug> GlobalConstraint<T> {
    pub fn new(check: fn(&Vec<Variable<T>>) -> bool) -> Self {
        GlobalConstraint { check }
    }
}

/// A generic constraint solver, with variables of type T
#[derive(Clone)]
pub struct ConstraintSolver<T: Eq + Hash + Clone + Debug> {
    variables: Vec<Variable<T>>,
    constraints: HashMap<usize, Vec<BinaryConstraint<T>>>,
    global_constraint: Option<GlobalConstraint<T>>,
}

impl<T: Eq + Hash + Clone + Debug> ConstraintSolver<T> {
    pub fn new(
        variables: Vec<Variable<T>>,
        constraints: HashMap<usize, Vec<BinaryConstraint<T>>>,
        global_constraint: Option<GlobalConstraint<T>>,
    ) -> Self {
        ConstraintSolver {
            variables,
            constraints,
            global_constraint,
        }
    }

    pub fn solve(self, finished: fn(&Vec<Variable<T>>) -> bool) -> Vec<Variable<T>> {
        for (i, variable) in self.variables.iter().enumerate() {
            for value in variable.domain.iter() {
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

    fn check_constrains(&mut self, last_set: usize) -> bool {
        // Check the global constraint
        if let Some(constraint) = &self.global_constraint {
            if !(constraint.check)(&self.variables) {
                return false;
            }
        }

        for constraint in self.constraints.get(&last_set).unwrap() {
            // Propagate constraints to neighbours
            if self.variables[constraint.right].value == None {
                let right = self.variables[constraint.right].clone();

                for value in right.domain.iter() {
                    if !((constraint.check)(
                        self.variables[constraint.left].value.as_ref().unwrap(),
                        value,
                    )) {
                        self.variables[constraint.right].domain.remove(value);

                        // If we've emptied a neighbouring domain, this is a failed assignment
                        if self.variables[constraint.right].domain.len() == 0 {
                            return false;
                        }
                    }
                }
            } else if !(constraint.check)(
                // Check assignment constraint
                self.variables[constraint.left].value.as_ref().unwrap(),
                self.variables[constraint.right].value.as_ref().unwrap(),
            ) {
                return false;
            }
        }

        true
    }
}
