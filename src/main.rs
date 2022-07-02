use crate::algrebra::*;

mod algrebra;

fn main() {
    let expr = Expression::Sum(
      Box::new(Expression::Group(Box::new(Expression::Sum(
            Box::new(Expression::Variable(1, "a".to_string())),
            Box::new(Expression::Variable(1, "b".to_string())),
        )))),
        Box::new(Expression::Variable(1, "c".to_string()))
    );

    println!("{}", expr);
}
