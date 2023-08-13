mod lib;

use crate::lib::{Context, Strategy, ShowStrategy1, ShowStrategy2};

fn main() {
    let mut context: Context<dyn Strategy> = Context::new(Box::new(ShowStrategy1));
    context.show();
    context.replace(Box::new(ShowStrategy2));
    context.show();
}
