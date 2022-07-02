use std::fmt::Display;

/// A mathematical expression
#[derive(Debug)]
pub enum Expression {
    Sum(Box<Sum>),
    Product(Box<Product>),
    Group(Box<Group>),
    Term(Box<Term>),
}

impl Expression {
  #[inline]
  pub fn sum(sum: Sum) -> Expression {
    Expression::Sum(Box::new(sum))
  }

  #[inline]
  pub fn product(product: Product) -> Expression {
    Expression::Product(Box::new(product))
  }

  #[inline]
  pub fn group(group: Group) -> Expression {
    Expression::Group(Box::new(group))
  }

  #[inline]
  pub fn term(term: Term) -> Expression {
    Expression::Term(Box::new(term))
  }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Sum(s) => write!(f, "{}", s),
            Expression::Product(p) => write!(f, "{}", p),
            Expression::Group(g) => write!(f, "{}", g),
            Expression::Term(t) => write!(f, "{}", t),
        }
    }
}

/// A base level term in a mathematical expression
#[derive(Debug)]
pub enum Term {
    Variable(String),
    Value(i64),
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Variable(v) => write!(f, "{}", v),
            Term::Value(v) => write!(f, "{}", v),
        }
    }
}

/// A parenthesized group of expressions
#[derive(Debug)]
pub struct Group {
  root: Expression,
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.root)
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
        write!(f, "{} + {}", self.left, self.right)
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
        write!(f, "{} * {}", self.left, self.right)
    }
}

impl Product {
    pub fn new(left: Expression, right: Expression) -> Self {
        Self { left, right }
    }
}

#[cfg(test)]
mod tests {
    use super::{Expression, Sum, Term};

    #[test]
    fn test_sum_simplify() {
        let expr = Sum::new(
            Expression::Term(Box::new(Term::Variable("x".to_string()))),
            Expression::Term(Box::new(Term::Variable("y".to_string()))),
        );
    }
}
