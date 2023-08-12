mod lib;

use crate::lib::{Expression, Context, Minus, Var};

fn main() {
    let a = Var::new("a");
    let b = Var::new("b");
    let c = Var::new("c");
    let mut context = Context::new();
    context.add(a.clone(), 10);
    context.add(b.clone(), 4);
    context.add(c.clone(), -3);

    let expression = Minus::new(Box::new(Minus::new(Box::new(a), Box::new(b))), Box::new(c));

    dbg!(&expression.show());

    let res = expression.interpret(&context);

    dbg!(res);
}
