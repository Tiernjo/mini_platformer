extern mod rsfml;
use rsfml::system::Vector2f;
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
	let wall_sprite = "../img/wall_5.png";
	let Avatar_sprite = "../img/player_5.png";

	let mut my_avatar = Avatar::new(Avatar_sprite);

	while window.is_open() {
		my_avatar.bounds = my_avatar.get_bounds();
		let mut grid:~[GridBlock] = ~[];
		let mut i = 0;
		// Make walls
		while i < 13 {
			grid.push(GridBlock::new(wall_sprite));
			match i {
				0..3	=>	{grid[i].set_position(i+6,3)}
				_		=>	{grid[i].set_position(i-3,5)}
			}
			
			grid[i].bounds = grid[i].get_bounds();
			if grid[i].bounds.top == my_avatar.bounds.height {
				my_avatar.force.x = 0.0;
			}
			i += 1;
		}
		i=0;
		let (is_x,move_by) = ::window::check(&mut window);
		match is_x{
			true	=>	{my_avatar.force.x = move_by}
			false	=>	{my_avatar.force.y = move_by}
		}
		
		my_avatar.walk(my_avatar.force.x);
		my_avatar.jump(my_avatar.force.y);
		my_avatar.force = Vector2f::new(0.0, 0.0);
		::render::game(&mut window, &background, grid, &my_avatar);
	}
}