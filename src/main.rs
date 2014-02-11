extern mod native;
extern mod rsfml;
mod render;
mod run;
mod window;

//Macs need rsfml to start on main thread
#[start]
fn start(argc: int, argv: **u8) -> int { 
    native::start(argc, argv, main)
}

fn main() {
	::run::main_loop();
}