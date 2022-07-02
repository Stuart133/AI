use std::fmt::Display;

/// A mathematical expression
#[derive(Debug)]
pub enum Expression {
    Sum(Box<Sum>),
    Product(Box<Product>),

    Term(String),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Sum(s) => write!(f, "{}", s),
            Expression::Product(p) => write!(f, "{}", p),
            Expression::Term(t) => write!(f, "{}", t),
        }
    }
}

/// A sum of two expressions
#[derive(Debug)]
pub struct Sum {
    left: Expression,
    right: Expression,
}

impl Sum {
    pub fn new(left: Expression, right: Expression) -> Self {
        Self { left, right }
    }
}

impl Display for Sum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + {})", self.left, self.right)
    }
}

/// A product of two expressions
#[derive(Debug)]
pub struct Product {
    left: Expression,
    right: Expression,
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} * {})", self.left, self.right)
    }
}

impl Product {
    pub fn new(left: Expression, right: Expression) -> Self {
        Self { left, right }
    }
}

#[cfg(test)]
mod tests {
    use super::{Expression, Sum};

    #[test]
    fn test_sum_simplify() {
        let expr = Sum::new(
            Expression::Term("x".to_string()),
            Expression::Term("y".to_string()),
        );
    }
}
