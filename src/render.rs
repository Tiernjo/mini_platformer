extern mod rsfml;
use rsfml::graphics::{Color,RenderWindow};
use rsfml::graphics::rc::{Sprite};

pub fn game(window:&mut RenderWindow, background:&Sprite) {
	window.clear(&Color::white());
	window.draw(background);
	window.display()
}