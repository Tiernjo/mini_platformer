extern mod rsfml;
use rsfml::system::{Clock,Time,Vector2f};
use enemy_spawner::MobSpawn;
use player::Avatar;
use ui::Menu;
use window;

pub fn main_loop() {
	// Window and Background
	let mut window = window::new();
	let background = window::background();

	// Variables
	let wall_sprite = "../img/wall.png";
	let avatar_sprite = "../img/player.png";
	let coin_sprite = "../img/coin.png";
	let goal_sprite = "../img/goal.png";
	let mut screen:uint = 0; let mut lives:f32 = 3.0;
	let mut coins_got:f32 = 0.0;

	// Pre loaded 
	// Avatar Init
	let mut my_avatar = Avatar::new(avatar_sprite);
	let avatar_origin = Vector2f::new(0.0,300.0);
	// Enemy system Init
	let mut enemy_spawn = MobSpawn::new();
	enemy_spawn.set_position(8,4);
	let mut enemy_time = Clock::new();
	// Walls Init
	let mut walls = ::world::set_wall(wall_sprite);	// set all walls
	walls.push(::world::set_coin(coin_sprite));	// Add coin to walls
	walls.push(::world::set_goal(goal_sprite));

	let mut overlay = ~[];
	overlay.push(Menu::new("Coins: 0"));
	overlay.push(Menu::new("Lives: 3"));
	overlay[0].set_position(&mut window, 0.1,0.9);
	overlay[1].set_position(&mut window,0.9,0.9);
	
	while window.is_open() {
		::window::check(&mut window);	//Window Close events
		match screen {
			// Title Screen
			0	=>	{
				screen = ::window::menu(screen);
				let mut menu = ~[];
				menu.push(Menu::new("Welcome to Mini Platformer"));
				menu.push(Menu::new("Press Space to Continue"));
				menu.push(Menu::new("Standard WASD Controls"));
				menu[0].set_position(&mut window, 0.5,0.25);
				menu[1].set_position(&mut window, 0.5,0.50);
				menu[2].set_position(&mut window, 0.5,0.75);
				// Re initialize variables
				lives = 3.0;
				my_avatar.set_position(avatar_origin);	//Avatar reset
				walls[32].set_position(3,2);	//Coin reset
				coins_got = 0.0;
				::world::set_enemy_spawn(&mut enemy_spawn);
				enemy_time.restart();
				::render::menu(&mut window,&background, menu);
			}
			// Game Screen
			1   =>	{
				// Controls
				::window::input(&mut my_avatar);
				// Collision
				::world::check_walls(&mut walls, &mut my_avatar);	// Player wall collision
				::world::check_walls(&mut walls, &mut enemy_spawn.mob);	// Enemy wall Collision
				let is_collected = ::world::check_coin(&mut my_avatar,&mut walls[32]);	// Whether coin is collected
				let is_dead = ::world::check_enemy(&mut my_avatar,&mut enemy_spawn.mob);	// Whether player and enemy touch
				let is_won = ::world::check_goal(&mut my_avatar,&mut walls[33]);
				// Respawn only mob
				if enemy_time.get_elapsed_time() > Time::with_seconds(5.5){
					::world::set_enemy_spawn(&mut enemy_spawn);
					enemy_time.restart();
				}
				// Check Collection of coin
				if is_collected{	
				walls[32].set_position(0,0); 
				coins_got = 1.0;
				}
				let coin_str = coins_got.to_str();
				overlay[0].set_text("Coins: "+coin_str);
				// Go to win screen	
				if is_won{screen = 3}
				// Game over scenario
				if is_dead {
					my_avatar.respawn(avatar_origin);
					::world::set_enemy_spawn(&mut enemy_spawn);
					lives -= 1.0;
					if lives < 0.0 {screen = 2}	
				}
				let live_string = lives.to_str();
				overlay[1].set_text("Lives: " + live_string);

				// Move/Jump player and monster by their recieved forces
				my_avatar.walk(my_avatar.force.x);my_avatar.jump(my_avatar.force.y);
				my_avatar.force = Vector2f::new(0.0, 10.0);
				enemy_spawn.mob.walk(enemy_spawn.mob.force.x); enemy_spawn.mob.jump(enemy_spawn.mob.force.y);
				enemy_spawn.mob.force = Vector2f::new(-2.0,10.0);
				::render::game(&mut window, &background, &mut walls, &my_avatar,&enemy_spawn.mob,&overlay);
			}
			// Game Over Screen
			2	=>	{
				screen = ::window::menu(screen);
				let mut menu = ~[];
				menu.push(Menu::new("You Lose"));
				menu.push(Menu::new("Press Space to Restart"));
				menu[0].set_position(&mut window, 0.5,0.25);
				menu[1].set_position(&mut window, 0.5,0.50);
				::render::menu(&mut window,&background,menu);
			}
			// You Won Screen
			3   =>	{
				screen = ::window::menu(screen);
				let mut menu = ~[];
				menu.push(Menu::new("You Win"));
				menu.push(Menu::new("Press Space to Restart"));
				menu[0].set_position(&mut window, 0.5,0.25);
				menu[1].set_position(&mut window, 0.5,0.50);
				::render::menu(&mut window,&background,menu);
			}
			_	=>	{}
		}
		
	}
}