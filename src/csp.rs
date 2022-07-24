use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone)]
pub struct Variable<T: Eq + Hash + Clone> {
    name: String,
    value: Option<T>,
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
    check: fn(&T, &T) -> bool,
}

/// A generic constraint solver, with variables of type T
#[derive(Clone)]
pub struct ConstraintSolver<T: Eq + Hash + Clone> {
    variables: Vec<Variable<T>>,
    constraints: HashMap<(usize, usize), BinaryConstraint<T>>,
}

impl<T: Eq + Hash + Clone> ConstraintSolver<T> {
    pub fn new(
        variables: Vec<Variable<T>>,
        constraints: HashMap<(usize, usize), BinaryConstraint<T>>,
    ) -> Self {
        ConstraintSolver {
            variables,
            constraints,
        }
    }

    pub fn solve(self) -> Vec<Variable<T>> {
        for (i, variable) in self.variables.iter().enumerate() {
            for value in variable.domain.iter() {
                let mut new_csp = self.clone();
                new_csp.variables[i].assign(value);

                // Check constraints
                if !self.check_constrains() {
                    continue;
                }

                // Check if we're finished
                let mut finished = true;
                for variable in new_csp.variables.iter() {
                    finished = finished && variable.is_assigned();
                }
                if finished {
                    return self.variables;
                }

                // Continue DFS
                let solved = new_csp.solve();
                if solved.len() != 0 {
                    return solved;
                }
            }
        }

        // Maybe a result here>
        vec![]
    }

    fn check_constrains(&self) -> bool {
        let mut valid = true;

        for (variables, constraint) in self.constraints.iter() {
            if self.variables[variables.0].is_assigned()
                && self.variables[variables.1].is_assigned()
            {
                valid = valid
                    && (constraint.check)(
                        self.variables[variables.0].value.as_ref().unwrap(),
                        self.variables[variables.1].value.as_ref().unwrap(),
                    );
            }
        }

        valid
    }
}
