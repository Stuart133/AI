use std::fmt::Display;

/// A mathematical expression
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Expression {
    Sum(Vec<Self>),
    Product(Vec<Self>),
    Integer(i64),
    Variable(i64, String),
}

#[allow(dead_code)]
impl Expression {
    pub fn simplify(self) -> Self {
        match self {
            Expression::Sum(sum) => {
                let mut new_sum = vec![];

                for expr in sum {
                    match expr {
                        Expression::Sum(mut s) => new_sum.append(&mut s), // Adjacent sums can be merged
                        _ => new_sum.push(expr.simplify()),
                    }
                }

                Expression::Sum(new_sum)
            }
            Expression::Product(product) => {
                let mut new_product = vec![];

                let mut iter = product.into_iter().peekable();
                while let Some(expr) = iter.next() {
                    match expr {
                        Expression::Product(mut p) => new_product.append(&mut p), // Adjacent products can be merged
                        Expression::Sum(s) => {
                            // If the next expression is also a sum, they can be merged via the distributive property
                            // (a + b) * (c + d) == ac + ad + bc + bd
                            if let Some(Expression::Sum(inner_s)) = iter.peek() {
                                // TODO: Tidy this up a bit
                                let mut terms = vec![];
                                for term in s {
                                    for inner_term in inner_s {
                                        terms.push(Expression::Product(vec![
                                            term.clone(),
                                            inner_term.clone(),
                                        ]))
                                    }
                                }

                                new_product.push(Expression::Sum(terms));

                                // We've already consumed the next term, so skip it now
                                iter.next();
                            } else {
                                new_product.push(Expression::Sum(s).simplify())
                            }
                        }
                        _ => new_product.push(expr.simplify()),
                    }
                }

                // If there is only one term, just return the term
                if new_product.len() > 1 {
                    Expression::Product(new_product)
                } else {
                    new_product[0].clone()
                }
            }
            _ => self,
        }
    }

    /// The total number of terms in the expression
    pub fn size(&self) -> u64 {
        match self {
            Expression::Sum(s) => s.iter().fold(0, |acc, expr| expr.size() + acc),
            Expression::Product(p) => p.iter().fold(0, |acc, expr| expr.size() + acc),
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
            Expression::Integer(_) => 0,
            Expression::Variable(_, _) => 0,
        }
    }

    fn precedence(&self) -> u64 {
        match self {
            Expression::Sum(_) => 2,
            Expression::Product(_) => 3,
            Expression::Integer(_) => 4,
            Expression::Variable(_, _) => 4,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Sum(terms) => write_terms(terms, self.precedence(), f, "+"),
            Expression::Product(terms) => write_terms(terms, self.precedence(), f, "*"),
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
    precedence: u64,
    f: &mut std::fmt::Formatter<'_>,
    sep: &'a str,
) -> std::fmt::Result {
    for i in 0..terms.len() - 1 {
        // Wrap the term in parenthesis if it's of lower precendence to preserve operation order
        if terms[i].precedence() < precedence {
            match write!(f, "({}) {} ", terms[i], sep) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        } else {
            match write!(f, "{} {} ", terms[i], sep) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        }
    }

    if terms[terms.len() - 1].precedence() < precedence {
        write!(f, "({})", terms[terms.len() - 1])
    } else {
        write!(f, "{}", terms[terms.len() - 1])
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::Expression;

    struct TestData {
        expr: Expression,
        written: String,
        depth: u64,
        size: u64,
        simplified_written: String,
        simplified_depth: u64,
        simplified_size: u64,
    }

    fn generate_test_data() -> Vec<TestData> {
        vec![
            TestData {
                expr: Expression::Product(vec![
                    Expression::Sum(vec![
                        Expression::Variable(1, "a".to_string()),
                        Expression::Variable(1, "b".to_string()),
                    ]),
                    Expression::Sum(vec![
                        Expression::Variable(1, "c".to_string()),
                        Expression::Variable(1, "d".to_string()),
                    ]),
                    Expression::Variable(1, "e".to_string()),
                ]),
                written: "(a + b) * (c + d) * e".to_string(),
                depth: 2,
                size: 5,
                simplified_written: "(a * c + a * d + b * c + b * d) * e".to_string(),
                simplified_depth: 3,
                simplified_size: 9,
            },
            TestData {
                expr: Expression::Product(vec![
                    Expression::Sum(vec![
                        Expression::Integer(1),
                        Expression::Variable(1, "a".to_string()),
                        Expression::Integer(10),
                    ]),
                    Expression::Product(vec![
                        Expression::Integer(10),
                        Expression::Variable(2, "b".to_string()),
                    ]),
                ]),
                written: "(1 + a + 10) * 10 * 2b".to_string(),
                depth: 2,
                size: 5,
                simplified_written: "(1 + a + 10) * 10 * 2b".to_string(),
                simplified_depth: 2,
                simplified_size: 5,
            },
            TestData {
                expr: Expression::Sum(vec![
                    Expression::Sum(vec![
                        Expression::Variable(1, "a".to_string()),
                        Expression::Variable(1, "b".to_string()),
                    ]),
                    Expression::Variable(1, "c".to_string()),
                    Expression::Sum(vec![
                        Expression::Variable(2, "d".to_string()),
                        Expression::Integer(25),
                    ]),
                ]),
                written: "a + b + c + 2d + 25".to_string(),
                depth: 2,
                size: 5,
                simplified_written: "a + b + c + 2d + 25".to_string(),
                simplified_depth: 1,
                simplified_size: 5,
            },
        ]
    }

    #[test]
    fn expression_write() {
        for data in generate_test_data() {
            let mut written = String::new();
            write!(&mut written, "{}", data.expr)
                .expect("Error occured while trying to write to written");

            assert_eq!(data.written, written);
        }
    }

    #[test]
    fn expression_size() {
        for data in generate_test_data() {
            assert_eq!(data.size, data.expr.size());
        }
    }

    #[test]
    fn expression_depth() {
        for data in generate_test_data() {
            assert_eq!(data.depth, data.expr.depth());
        }
    }

    #[test]
    fn expression_simplify() {
        for data in generate_test_data() {
            let new_expr = data.expr.simplify();

            let mut written = String::new();
            write!(&mut written, "{}", new_expr)
                .expect("Error occured while trying to write to written");

            assert_eq!(data.simplified_size, new_expr.size());
            assert_eq!(data.simplified_depth, new_expr.depth());
            assert_eq!(data.simplified_written, written);
        }
    }
}
