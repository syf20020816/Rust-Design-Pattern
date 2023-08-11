mod lib;

use crate::lib::{AIHelper, Light, RiceCooker, ElectricalEquipment};

fn main() {
    let mut helper = AIHelper::new();
    helper.push(Box::new(Light::new("light 1")));
    helper.push(Box::new(RiceCooker::new("cooker 1")));
    helper.ctrl("cooker 1", true);
}
