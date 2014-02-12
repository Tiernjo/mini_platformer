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
	let avatar_sprite = "../img/player_5.png";
	let coin_sprite = "../img/coin.png";

	let mut my_avatar = Avatar::new(avatar_sprite);
	

	while window.is_open() {
		my_avatar.bounds = my_avatar.get_bounds();
		::window::check(&mut window);
		::window::input(&mut my_avatar);

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
			if grid[i].bounds.top == my_avatar.bounds.top +50.0 {	// If avatar's bottom touches block's top
				if grid[i].bounds.left < my_avatar.bounds.left  + 50.0{	// and if box is to the right of avatar
					if my_avatar.force.y > 0.0 {
						my_avatar.force.y = 0.0;			//avatar has no up/down force
					}
							
				}
			}
			if grid[i].bounds.left == my_avatar.bounds.left + 50.0 {	// if if avatar's right touches blocks left
				if grid[i].bounds.top < my_avatar.bounds.top +50.0{		//
					if grid[i].bounds.top +100.0 > my_avatar.bounds.top {	// and if avatar si within vicinity of box side
						my_avatar.force.x = 0.0;	// avatar has no left or right force
					}
				}
			}
	
			i += 1;
		}
		let mut only_coin = GridBlock::new(coin_sprite);
		only_coin.set_position(3,2);
		grid.push(only_coin);
		i=0;
		

		
		my_avatar.walk(my_avatar.force.x);
		my_avatar.jump(my_avatar.force.y);
		my_avatar.force = Vector2f::new(0.0, 1.0);
		::render::game(&mut window, &background, grid, &my_avatar);
	}
}