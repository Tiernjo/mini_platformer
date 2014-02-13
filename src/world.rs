use player::Avatar;
use wall::GridBlock;

pub fn set_wall(wall_sprite:&str, my_avatar:&mut Avatar) -> ~[GridBlock]{
	let mut grid = ~[];
	let mut i = 0;
	while i < 13 {
		grid.push(GridBlock::new(wall_sprite));
		match i {
			0..3	=>	{grid[i].set_position(i+6,3)}
			4..13	=>	{grid[i].set_position(i-3,5)}
			//14..17	=>	{grid[i].set_position(0, i - 13)}
			//18..22	=>	{grid[i].set_position(10, i - 17)}
			//23..31	=>	{grid[i].set_position(i -22,0)}
			_		=>	{}
		}
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
	grid
}