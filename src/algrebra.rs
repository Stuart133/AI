use std::fmt::Display;

/// A mathematical expression
#[derive(Debug)]
pub enum Expression {
    Sum(Vec<Self>),
    Product(Vec<Self>),
    Group(Box<Self>),
    Integer(i64),
    Variable(i64, String),
}

impl Expression {
    pub fn group(root: Self) -> Self {
      Expression::Group(Box::new(root))
    }


    pub fn simplify(&mut self) {
        match self {
            Expression::Sum(_) => todo!(),
            Expression::Product(_) => todo!(),
            Expression::Group(_) => todo!(),
            Expression::Integer(_) => {}
            Expression::Variable(_, _) => {}
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Sum(terms) => write_terms(terms, f, "+"),
            Expression::Product(terms) => write_terms(terms, f, "*"),
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

fn write_terms<'a>(
    terms: &Vec<Expression>,
    f: &mut std::fmt::Formatter<'_>,
    sep: &'a str,
) -> std::fmt::Result {
    for i in 0..terms.len() - 1 {
        write!(f, "{} + ", terms[i]);
    }

    write!(f, "{}", terms[terms.len() - 1])
}

#[cfg(test)]
mod tests {
    use super::Expression;

    #[test]
    fn test_sum_simplify() {
        let expr = Expression::Sum(
            Box::new(Expression::Group(Box::new(Expression::Sum(
                Box::new(Expression::Variable(1, "a".to_string())),
                Box::new(Expression::Variable(1, "b".to_string())),
            )))),
            Box::new(Expression::Variable(1, "c".to_string())),
        );
    }
}
