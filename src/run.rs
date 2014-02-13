extern mod rsfml;
use rsfml::system::{Clock,Time,Vector2f};
use enemy_spawner::MobSpawn;
use player::Avatar;
use window;

pub fn main_loop() {
	let mut window = window::new();
	let background = window::background();

	// textures
	let wall_sprite = "../img/wall_5.png";
	let avatar_sprite = "../img/player_6.png";
	let coin_sprite = "../img/coin.png";

	// Pre loaded 
	let mut enemy_time = Clock::new();
	let mut walls = ~[]; 
	let mut my_avatar = Avatar::new(avatar_sprite);
	my_avatar.set_position(Vector2f::new(0.0, 300.0));
	let mut enemy_spawn = MobSpawn::new();
	enemy_spawn.set_position(8,4);
	let mut enemy = ::world::set_enemy(&enemy_spawn);

	while window.is_open() {
		my_avatar.bounds = my_avatar.get_bounds();
		::window::check(&mut window);
		::window::input(&mut my_avatar);
		walls =::world::set_wall(wall_sprite, &mut my_avatar);	// Set walls and wall "phyicsooioioe"
		walls.push(::world::set_coin(coin_sprite));	// Set coin location
		let mut avatars = ~[&mut my_avatar];
		avatars.push(&mut enemy);

		// Set avatar movement and reset force
		avatars[0].walk(my_avatar.force.x);
		avatars[0].jump(my_avatar.force.y);
		avatars[0].force = Vector2f::new(0.0, 10.0);
		::render::game(&mut window, &background, walls, avatars);
	}
}