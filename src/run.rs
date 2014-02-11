extern mod rsfml;
mod render;
mod window;

pub fn main_loop() {
	let mut window = window::new();
	let background = window::background();

	while window.is_open() {
		let input = ::window::check(&mut window);
		::render::game(&mut window, &background);
	}
}