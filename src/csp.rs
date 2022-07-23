use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Variable<T> {
    name: String,
    value: Option<T>,
    domain: HashSet<T>,
}

#[derive(Debug)]
pub struct BinaryConstraint<T> {
    check: fn(T, T) -> bool,
}

/// A generic constraint solver, with variables of type T
#[derive(Debug)]
pub struct ConstraintSolver<T> {
    variables: Vec<Variable<T>>,
    constraints: HashMap<(String, String), BinaryConstraint<T>>,
}

impl<T> ConstraintSolver<T> {
    pub fn new(
        variables: Vec<Variable<T>>,
        constraints: HashMap<(String, String), BinaryConstraint<T>>,
    ) -> Self {
        ConstraintSolver {
            variables,
            constraints,
        }
    }
}
