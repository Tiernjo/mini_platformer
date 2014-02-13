extern mod rsfml;
use rsfml::system::Vector2f;
use player::Avatar;
use wall::GridBlock;
use window;
pub fn main_loop() {
	let mut window = window::new();
	let background = window::background();

	// textures
	let wall_sprite = "../img/wall_5.png";
	let avatar_sprite = "../img/player_5.png";
	let coin_sprite = "../img/coin.png";

	// Pre loaded 
	let mut grid = ~[];
	let mut my_avatar = Avatar::new(avatar_sprite);
	my_avatar.set_position(Vector2f::new(0.0, 300.0));

	while window.is_open() {
		my_avatar.bounds = my_avatar.get_bounds();
		::window::check(&mut window);
		::window::input(&mut my_avatar);
		grid =::world::set_wall(wall_sprite, &mut my_avatar);

		let mut only_coin = GridBlock::new(coin_sprite);
		only_coin.set_position(3,2);
		grid.push(only_coin);

		
		my_avatar.walk(my_avatar.force.x);
		my_avatar.jump(my_avatar.force.y);
		my_avatar.force = Vector2f::new(0.0, 10.0);
		::render::game(&mut window, &background, grid, &my_avatar);
	}
}