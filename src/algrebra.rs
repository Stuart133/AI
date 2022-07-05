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

    pub fn simplify(self) -> Self {
        match self {
            Expression::Sum(sum) => {
                let mut new_sum: Vec<Expression> = vec![];

                for expr in sum {
                    match expr {
                        Expression::Sum(mut s) => new_sum.append(&mut s), // Adjacent sums can be merged
                        Expression::Group(g) => {
                            // Apply associative rule to adjacent grouped sums: a + (b + c) == a + b + c
                            if let Expression::Sum(mut s) = *g {
                                new_sum.append(&mut s)
                            } else {
                                new_sum.push(g.simplify())
                            }
                        }
                        _ => {
                            new_sum.push(expr.simplify())
                        }
                    }
                }

                Expression::Sum(new_sum)
            }
            Expression::Group(g) => g.simplify(),
            // TODO: Simplify product
            _ => self
        }
    }

    /// The total number of terms in the expression
    pub fn size(&self) -> u64 {
        match self {
            Expression::Sum(s) => s.iter().fold(0, |acc, expr| expr.size() + acc),
            Expression::Product(p) => p.iter().fold(0, |acc, expr| expr.size() + acc),
            Expression::Group(g) => g.size(),
            Expression::Integer(_) => 1,
            Expression::Variable(_, _) => 1,
        }
    }

    /// The depth of the full expression tree
    pub fn depth(&self) -> u64 {
        match self {
            Expression::Sum(s) => {
                let depths = s.iter().map(|expr| expr.depth());

                match depths.max() {
                    Some(max) => max + 1,
                    None => 0, // If the expression has no terms then it doesn't contribute to depth
                }
            }
            Expression::Product(p) => {
                let depths = p.iter().map(|expr| expr.depth());

                match depths.max() {
                    Some(max) => max + 1,
                    None => 0, // If the expression has no terms then it doesn't contribute to depth
                }
            }
            Expression::Group(g) => g.depth(),
            Expression::Integer(_) => 0,
            Expression::Variable(_, _) => 0,
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
        match write!(f, "{} {} ", terms[i], sep) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };
    }

    write!(f, "{}", terms[terms.len() - 1])
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::Expression;

    #[test]
    fn expression_write() {
        let expr = Expression::Product(vec![
            Expression::Sum(vec![
                Expression::Integer(1),
                Expression::Variable(1, "a".to_string()),
                Expression::Integer(10),
            ]),
            Expression::group(Expression::Product(vec![
                Expression::Integer(10),
                Expression::Variable(2, "b".to_string()),
            ])),
        ]);

        let mut written = String::new();
        write!(&mut written, "{}", expr).expect("Error occured while trying to write to written");

        assert_eq!("1 + a + 10 * (10 * 2b)", written);
    }

    #[test]
    fn expression_size() {
        let expr = Expression::Product(vec![
            Expression::Sum(vec![
                Expression::Integer(1),
                Expression::Variable(1, "a".to_string()),
                Expression::Integer(10),
            ]),
            Expression::group(Expression::Product(vec![
                Expression::Integer(10),
                Expression::Variable(2, "b".to_string()),
            ])),
        ]);

        assert_eq!(5, expr.size());
    }

    #[test]
    fn expression_depth() {
        let expr = Expression::Product(vec![
            Expression::Sum(vec![
                Expression::Integer(1),
                Expression::Variable(1, "a".to_string()),
                Expression::Integer(10),
            ]),
            Expression::group(Expression::Product(vec![
                Expression::Integer(10),
                Expression::Variable(2, "b".to_string()),
            ])),
        ]);

        assert_eq!(2, expr.depth());
    }

    #[test]
    fn sum_simplify() {
        let mut expr = Expression::Sum(vec![
            Expression::group(Expression::Sum(vec![
                Expression::Variable(1, "a".to_string()),
                Expression::Variable(1, "b".to_string()),
            ])),
            Expression::Variable(1, "c".to_string()),
        ]);

        expr = expr.simplify();

        assert_eq!(3, expr.size());
        assert_eq!(1, expr.depth());
    }
}
