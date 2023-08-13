mod lib;

use crate::lib::{Visitor, ConcreteVisitor, Element, ElementA, ElementB};

fn main() {
    let eles: Vec<Box<dyn Element>> = vec![
        Box::new(ElementA), Box::new(ElementB),
    ];
    let visitor = ConcreteVisitor;
    for ele in eles {
        ele.accept(&visitor);
    }
}
