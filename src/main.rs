use crate::algrebra::*;

mod algrebra;

fn main() {
    let expr = Sum::new(
        Expression::Term("x".to_string()),
        Expression::Product(Box::new(Product::new(
            Expression::Term("y".to_string()),
            Expression::Term("z".to_string()),
        ))),
    );

    println!("{}", expr);
}
