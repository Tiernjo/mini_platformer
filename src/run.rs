extern mod rsfml;
use rsfml::system::{Clock,Time,Vector2f};
use enemy_spawner::MobSpawn;
use player::Avatar;
use wall::GridBlock;
use window;

pub fn main_loop() {
	let mut window = window::new();
	let background = window::background();

	// textures
	let wall_sprite = "../img/wall_5.png";
	let avatar_sprite = "../img/player_6.png";
	let coin_sprite = "../img/coin.png";

	// Pre loaded 
	// Avatar Init
	let mut my_avatar = Avatar::new(avatar_sprite);
	my_avatar.set_position(Vector2f::new(0.0, 300.0));
	// Enemy system Init
	let mut enemy_spawn = MobSpawn::new();
	enemy_spawn.set_position(8,4);
	let mut enemy = ::world::set_enemy(&enemy_spawn);
	let mut enemy_time = Clock::new();
	// Walls Init
	let mut walls = ::world::set_wall(wall_sprite);	// set all walls
	walls.push(::world::set_coin(coin_sprite));	// Add coin to walls
	
	while window.is_open() {
		::window::check(&mut window);
		::window::input(&mut my_avatar);
		::world::check_walls(&mut walls, &mut my_avatar);

		my_avatar.walk(my_avatar.force.x);
		my_avatar.jump(my_avatar.force.y);
		my_avatar.force = Vector2f::new(0.0, 10.0);
		::render::game(&mut window, &background, &mut walls, &my_avatar);
	}
}