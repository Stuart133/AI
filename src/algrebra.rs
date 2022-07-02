use std::fmt::Display;

/// A mathematical expression
#[derive(Debug)]
pub enum Expression {
    Sum(Box<Self>, Box<Self>),
    Product(Box<Self>, Box<Self>),
    Group(Box<Self>),
    Integer(i64),
    Variable(i64, String),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Sum(l, r) => write!(f, "{} + {}", l, r),
            Expression::Product(l, r) => write!(f, "{} * {}", l, r),
            Expression::Group(g) => write!(f, "({})", g),
            Expression::Integer(i) => write!(f, "{}", i),
            Expression::Variable(c, v) => {
                if *c == 1 {
                    write!(f, "{}", v)
                } else {
                    write!(f, "{}{}", c, v)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Expression};

    #[test]
    fn test_sum_simplify() {}
}
