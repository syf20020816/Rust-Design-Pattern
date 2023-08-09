mod lib;

use crate::lib::{Linux, Mac, System, MP4, VideoPlayer};

fn main() {
    let mac = Mac::new(Box::new(MP4::new()));
    mac.play("小猪佩奇");
}
