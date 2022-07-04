use crate::algrebra::*;

mod algrebra;

fn main() {
    let expr = Expression::Sum(vec![
        Expression::group(Expression::Sum(vec![
            Expression::Variable(1, "a".to_string()),
            Expression::Variable(1, "b".to_string()),
        ])),
        Expression::Variable(1, "c".to_string()),
    ]);

    println!("{}", expr);
}
