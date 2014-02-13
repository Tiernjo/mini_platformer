extern mod native;
extern mod rsfml;

pub mod enemy_spawner;
pub mod player;
pub mod render;
pub mod run;
pub mod ui;
pub mod wall;
pub mod window;
pub mod world;

//Macs need rsfml to start on main thread
#[start]
fn start(argc: int, argv: **u8) -> int { 
    native::start(argc, argv, main)
}
#[main]
fn main() {
	::run::main_loop();
}