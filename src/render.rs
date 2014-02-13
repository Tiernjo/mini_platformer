extern mod rsfml;
use rsfml::graphics::{Color,RenderWindow};
use rsfml::graphics::rc::{Sprite};
use player::Avatar;
use ui::Menu;
use wall::GridBlock;

pub fn game(window:&mut RenderWindow, background:&Sprite,grid:&mut ~[GridBlock],my_avatar:&Avatar,enemy:&Avatar,menu:&~[Menu]) {
	window.clear(&Color::white());
	window.draw(background);
	for contents in grid.iter() {
		window.draw(contents);
	}
	window.draw(my_avatar);window.draw(enemy);
	for contents in menu.iter() {
		window.draw(contents);
	}
	window.display()
}
pub fn menu (window:&mut RenderWindow,background:&Sprite,menu:~[Menu]) {
	window.clear(&Color::white());
	window.draw(background);
	for contents in menu.iter() {
		window.draw(contents);
	}
	window.display()
}