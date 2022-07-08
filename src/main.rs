use crate::algrebra::*;

mod algrebra;
pub mod graph;

fn main() {
    let expr = Expression::Sum(vec![
        Expression::Sum(vec![
            Expression::Variable(1, "a".to_string()),
            Expression::Variable(1, "b".to_string()),
        ]),
        Expression::Variable(1, "c".to_string()),
    ]);

    println!("{}", expr);
}
