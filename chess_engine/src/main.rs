mod tests;

use env_logger::{self};
use log::info;

mod engine;
use engine::EngineController;

fn main() {
    env_logger::init();

    info!("Creating controller");
    let mut controller = EngineController::default();
    controller.set_black(engine::Player::Manual);

    info!("Initiating game");
    controller.play().unwrap();
}
