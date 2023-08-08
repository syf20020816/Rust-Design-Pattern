mod lib;

use crate::lib::{Adapter,System};

fn main() {
    System::run_api(Adapter);
}
