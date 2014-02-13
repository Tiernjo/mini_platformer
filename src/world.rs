use enemy_spawner::MobSpawn;
use player::Avatar;
use wall::GridBlock;

pub fn set_wall(wall_sprite:&str) -> ~[GridBlock]{
	let mut grid = ~[];
	let mut i = 0;
	while i < 32 {
		grid.push(GridBlock::new(wall_sprite));
		match i {
			0..3	=>	{grid[i].set_position(i+6,3)}
			4..13	=>	{grid[i].set_position(i-3,5)}
			14..17	=>	{grid[i].set_position(0, i - 13)}
			18..22	=>	{grid[i].set_position(10, i - 17)}
			23..31	=>	{grid[i].set_position(i -22,0)}
			_		=>	{}
		}
		i += 1;
	}
	grid
}
pub fn check_walls(grid:&mut ~[GridBlock],my_avatar:&mut Avatar) {
	let mut i = 0;
	while i < 32 {
		my_avatar.bounds = my_avatar.get_bounds();
		grid[i].bounds = grid[i].get_bounds();
		// Set top solidity
		if grid[i].bounds.top == my_avatar.bounds.top +50.0 {	// If avatar's bottom touches block's top
			if grid[i].bounds.left < my_avatar.bounds.left  + 50.0{	// and if box is to the right of avatar
				if grid[i].bounds.left + 100.0 > my_avatar.bounds.left {
					if my_avatar.force.y > 0.0 {
						my_avatar.force.y = 0.0;
					}
				}
			}
		}
		// Set bottom solidity
		if grid[i].bounds.top +100.0 == my_avatar.bounds.top {	// If avatar's top touches block's bottom
			if grid[i].bounds.left < my_avatar.bounds.left + 50.0{	// and if box is to the right of avatar
				if grid[i].bounds.left + 100.0 > my_avatar.bounds.left {
					if my_avatar.force.y < 0.0 {
						my_avatar.force.y = 0.0;
					}
				}
			}
		}
		// Set left solidity
		if grid[i].bounds.left == my_avatar.bounds.left + 50.0 {	// if if avatar's right touches blocks left
			if grid[i].bounds.top < my_avatar.bounds.top +50.0{		//
				if grid[i].bounds.top +100.0 > my_avatar.bounds.top {	// and if avatar is within vicinity of box side
					if my_avatar.force.x > 0.0 {
						my_avatar.force.x = 0.0;	// avatar has no left or right force
					}
				}
			}
		}
		// Set right solidity
		if grid[i].bounds.left + 100.0 == my_avatar.bounds.left {	// if avatar's left touches block right
			if grid[i].bounds.top < my_avatar.bounds.top + 50.0 {	//
				if grid[i].bounds.top + 100.0 > my_avatar.bounds.top {
					if my_avatar.force.x < 0.0 {
						my_avatar.force.x = 0.0;
					}
				}
			}
		}	
		i += 1;
	}
}
pub fn check_coin(player:&Avatar,coin:&mut GridBlock) -> bool {
	let mut is_taken = false;
	coin.bounds = coin.get_bounds();
	if player.bounds.top <= coin.bounds.top +100.0 && player.bounds.left <= coin.bounds.left +100.0
	&& player.bounds.top +50.0 >= coin.bounds.top && player.bounds.left +50.0 >= coin.bounds.left{
		is_taken = true;
	}
	is_taken
}
pub fn set_coin(coin_sprite:&str) -> GridBlock{
	let mut only_coin = GridBlock::new(coin_sprite);
	only_coin.set_position(3,2);
	only_coin
}
pub fn check_goal(player:&Avatar,goal:&mut GridBlock) -> bool {
	let mut is_won = false;
	goal.bounds = goal.get_bounds();
	if player.bounds.top <= goal.bounds.top +200.0 && player.bounds.left <= goal.bounds.left +200.0
	&& player.bounds.top +50.0 >= goal.bounds.top && player.bounds.left +50.0 >= goal.bounds.left{
		is_won = true;
	}
	is_won
}
pub fn set_goal(goal_sprite:&str)	-> GridBlock{
	let mut goal = GridBlock::new(goal_sprite);
	goal.set_position(9,1);
	goal
}
pub fn check_enemy(player:&Avatar,enemy:&Avatar) -> bool{
	let mut is_touching = false;
	if player.bounds.top <= enemy.bounds.top +100.0 && player.bounds.left <= enemy.bounds.left +100.0
	&& player.bounds.top +50.0 >= enemy.bounds.top && player.bounds.left +50.0 >= enemy.bounds.left{
		is_touching = true;
	}
	is_touching
}
pub fn set_enemy_spawn(enemy_spawn:&mut MobSpawn) {
	let enemy_loc = enemy_spawn.get_position();
	enemy_spawn.mob.respawn(enemy_loc);
}