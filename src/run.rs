extern mod rsfml;
use wall::GridBlock;
use player::Avatar;
mod player;
mod render;
mod wall;
mod window;


pub fn main_loop() {
	let mut window = window::new();
	let background = window::background();

	// textures
	let wall_sprite = "../img/wall_2.png";
	let Avatar_sprite = "../img/player.png";

	while window.is_open() {
		let mut grid:~[GridBlock] = ~[];
		let mut i = 0;
		// Make walls
		while i < 13 {
			grid.push(GridBlock::new(wall_sprite));
			match i {
				0..3	=>	{grid[i].set_position(i+6,2)}
				_		=>	{grid[i].set_position(i-3,4)}
			}
			i += 1;
		}
		i=0;
		let mut my_avatar = Avatar::new(Avatar_sprite);
		let input = ::window::check(&mut window);
		::render::game(&mut window, &background, grid, my_avatar);
	}
}