mod lib;

use crate::lib::{Shape, ShapeFactory};

fn main() {
    let factory = ShapeFactory::new();
    ///从这里获取的其实是个新的Shape
    let mut shape = factory.get("rect");
    shape.show();
    shape.add_style("font-size", "16px");
    shape.show();
}
