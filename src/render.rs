extern mod rsfml;
use rsfml::graphics::{Color,RenderWindow};
use rsfml::graphics::rc::{Sprite};
use player::Avatar;
use wall::GridBlock;
mod wall;
mod player;


pub fn game(window:&mut RenderWindow, background:&Sprite,grid:~[GridBlock],player:&Avatar) {
	window.clear(&Color::white());
	window.draw(background);
	for contents in grid.iter() {
		window.draw(contents);
	}
	window.draw(player);
	window.display()
}