extern mod rsfml;
use rsfml::graphics::{RenderWindow,Texture};
use rsfml::graphics::rc::{Sprite};
use rsfml::window::{Close,ContextSettings,event,keyboard,VideoMode};
use std::cell::RefCell;
use std::rc::Rc;

pub fn new() -> RenderWindow{
	let settings = ContextSettings::default();
	let window = match RenderWindow::new(VideoMode::new_init(900, 600, 32),"Mini Platformer",Close,&settings) {
		Some(window)	=>	window,
		None()			=>	fail!(~"Error, making window")
	};
	window
}

pub fn background() -> Sprite {
	let back_texture = match Texture::new_from_file("../img/background.png"){
		Some(back_texture)	=>	back_texture,
		None()				=>	fail!(~"Error, making background texture.")
	};
	let back_ref_cell = RefCell::new(back_texture);
	let back_rc = Rc::new(back_ref_cell);
	let back_sprite = match Sprite::new_with_texture(back_rc) {
		Some(back_sprite)	=>	back_sprite,
		None()				=>	fail!(~"Error, making background sprite.")
	};
	back_sprite
}

pub fn check(window:&mut RenderWindow) ->	~str {
	let mut input = ~"";
	loop {
		match window.poll_event() {
			event::Closed		=>	window.close(),
			event::KeyPressed{code, ..}	=>	match code {
				keyboard::D 	=>	{return ~"right"}
				keyboard::A		=>	{return ~"left"}
				keyboard::Space	=>	{return ~"jump"}
				_				=>	{}
			},
			event::NoEvent		=>	break,
			_					=>	{}
		}
	}
	input
}