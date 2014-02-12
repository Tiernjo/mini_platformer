extern mod native;
extern mod rsfml;
mod player;
mod render;
mod run;
mod wall;
mod window;

//Macs need rsfml to start on main thread
#[start]
fn start(argc: int, argv: **u8) -> int { 
    native::start(argc, argv, main)
}

fn main() {
	::run::main_loop();
}