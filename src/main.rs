use crate::algrebra::*;

mod algrebra;

fn main() {
    let expr = Sum::new(
        Expression::term(Term::Variable("x".to_string())),
        Expression::product(Product::new(
            Expression::term(Term::Variable("y".to_string())),
            Expression::term(Term::Value(10)),
        )),
    );

    println!("{}", expr);
}
