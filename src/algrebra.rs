use std::fmt::{Display, LowerExp};

/// A mathematical expression
#[derive(Debug)]
pub enum Expression {
    Sum(Box<Expression>, Box<Expression>),
    Product(Box<Expression>, Box<Expression>),
    Group(Box<Expression>),
    Term(Box<Term>),
}

impl Expression {
  #[inline]
  pub fn sum(left: Expression, right: Expression) -> Expression {
    Self::Sum(Box::new(left), Box::new(right))
  }

  #[inline]
  pub fn product(left: Expression, right: Expression) -> Expression {
    Self::Product(Box::new(left), Box::new(right))
  }

  #[inline]
  pub fn group(root: Expression) -> Expression {
    Self::Group(Box::new(root))
  }

  #[inline]
  pub fn term(term: Term) -> Expression {
    Self::Term(Box::new(term))
  }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Sum(l, r) => write!(f, "{} + {}", l, r),
            Expression::Product(l, r) => write!(f, "{} * {}", l, r),
            Expression::Group(g) => write!(f, "({})", g),
            Expression::Term(t) => write!(f, "{}", t),
        }
    }
}

/// A base level term in a mathematical expression
#[derive(Debug)]
pub enum Term {
  /// A variable term, composed of an integer coefficient & a label
    Variable(i64, String),

    /// A constant integer term
    Constant(i64),
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Variable(c, v) => write!(f, "{}{}", c, v),
            Term::Constant(v) => write!(f, "{}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Expression, Term};

    #[test]
    fn test_sum_simplify() {
    }
}
