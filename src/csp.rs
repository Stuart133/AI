use std::{collections::{HashMap, HashSet}, hash::Hash};

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
}

#[derive(Debug, Clone)]
pub struct BinaryConstraint<T> {
    check: fn(T, T) -> bool,
}

/// A generic constraint solver, with variables of type T
#[derive(Debug, Clone)]
pub struct ConstraintSolver<T: Eq + Hash + Clone> {
    variables: Vec<Variable<T>>,
    constraints: HashMap<(String, String), BinaryConstraint<T>>,
}

impl<T: Eq + Hash + Clone> ConstraintSolver<T> {
    pub fn new(
        variables: Vec<Variable<T>>,
        constraints: HashMap<(String, String), BinaryConstraint<T>>,
    ) -> Self {
        ConstraintSolver {
            variables,
            constraints,
        }
    }

    pub fn solve(&self) -> Vec<Variable<T>> {
        for (i, variable) in self.variables.iter().enumerate() {
          for value in variable.domain.iter() {
            let mut new_csp = self.clone();
            new_csp.variables[i].assign(value);

            // Check constraints

            // Check if we're finished

            // Continue DFS
            return new_csp.solve();
          }
        }

        vec![]
    }
}
