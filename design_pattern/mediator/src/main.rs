mod lib;

use crate::lib::{RealEstateManager, Person, Mediator, HouseBuyer, HouseOwner};

fn main() {
    let mut owner = HouseOwner::new("owner");
    let mut buyer = HouseBuyer::new("buyer");
    let mediator = RealEstateManager::new(&owner, &buyer);
    owner.chat(&mediator, "Hello ! Does someone wanna buy my house?");
    buyer.chat(&mediator, "I may wanna buy !")
}
