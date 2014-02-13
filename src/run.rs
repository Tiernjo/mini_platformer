extern mod rsfml;
use rsfml::system::{Clock,Time,Vector2f};
use enemy_spawner::MobSpawn;
use player::Avatar;
use window;

pub fn main_loop() {
	// Window and Background
	let mut window = window::new();
	let background = window::background();

	// textures
	let wall_sprite = "../img/wall_5.png";
	let avatar_sprite = "../img/player_7.png";
	let coin_sprite = "../img/coin.png";

	// Pre loaded 
	// Avatar Init
	let mut my_avatar = Avatar::new(avatar_sprite);
	let avatar_origin = Vector2f::new(0.0,300.0);
	my_avatar.set_position(avatar_origin);
	// Enemy system Init
	let mut enemy_spawn = MobSpawn::new();
	enemy_spawn.set_position(8,4);
	let mut enemy_time = Clock::new();
	// Walls Init
	let mut walls = ::world::set_wall(wall_sprite);	// set all walls
	walls.push(::world::set_coin(coin_sprite));	// Add coin to walls
	// 
	let mut screen:uint = 1; let mut lives = 3;
	
	while window.is_open() {
		::window::check(&mut window);	//Window Close events
		match screen {
			0	=>	{}
			1   =>	{
				// Controls
				::window::input(&mut my_avatar);
				// Collision
				::world::check_walls(&mut walls, &mut my_avatar);	// Player wall collision
				::world::check_walls(&mut walls, &mut enemy_spawn.mob);	// Enemy wall Collision
				let is_collected = ::world::check_coin(&mut my_avatar,&mut walls[32]);	// Whether coin is collected
				let is_dead = ::world::check_enemy(&mut my_avatar,&mut enemy_spawn.mob);	// Whether player and enemy touch
				// Respawn only mob
				if enemy_time.get_elapsed_time() > Time::with_seconds(5.5){
					::world::set_enemy_spawn(&mut enemy_spawn);
					enemy_time.restart();
				}
				if is_collected{walls[32].set_position(0,0)}	// Check Collection of coin
				if is_dead {
					my_avatar.respawn(avatar_origin);
					::world::set_enemy_spawn(&mut enemy_spawn);
					lives -= 1;
					if lives < 0 {screen = 2}	
				}	

				// Move/Jump player and monster by their recieved forces
				my_avatar.walk(my_avatar.force.x);my_avatar.jump(my_avatar.force.y);
				my_avatar.force = Vector2f::new(0.0, 10.0);
				enemy_spawn.mob.walk(enemy_spawn.mob.force.x); enemy_spawn.mob.jump(enemy_spawn.mob.force.y);
				enemy_spawn.mob.force = Vector2f::new(-2.0,10.0);
				::render::game(&mut window, &background, &mut walls, &my_avatar,&enemy_spawn.mob);
			}
			2	=>	{
				::render::menu(&mut window,&background);
			}
			_	=>	{}
		}
		
	}
}