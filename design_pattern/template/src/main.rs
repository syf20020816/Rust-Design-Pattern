mod lib;

use crate::lib::{MobaGame, GameTemplate, SportGame};

fn main() {
    let game = MobaGame::init();
    game.start();
    let running = "running";
    game.end(|| -> (){
        println!("{}", running);
        println!("{} is winner", &game.win(true));
    })
}
