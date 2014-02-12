extern mod rsfml;
use rsfml::graphics::{RenderTexture,RenderWindow,Texture};
use rsfml::graphics::rc::{Sprite};
use rsfml::system::Vector2f;
use rsfml::traits::Drawable;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Avatar<'p>{
	body:Sprite,
	position:Vector2f,
}

impl<'p> Avatar<'p> {
	pub fn new(texture_loc:&str) -> Avatar<'p> {
		let player_texture = match Texture::new_from_file(texture_loc){
			Some(player_texture)	=>	player_texture,
			None()					=>	fail!("Error, Player texture."),
		};
		let texture_ref_cell = RefCell::new(player_texture);
		let texture_rc = Rc::new(texture_ref_cell);
		let player_sprite = match Sprite::new_with_texture(texture_rc){
			Some(player_sprite)	=>	player_sprite,
			None()				=>	fail!("Error, Player sprite."),
		};
		Avatar{body:player_sprite,position:Vector2f::new(0.0,0.0)}
	}
}

// Make WallBlock drawable
impl<'p> Drawable for Avatar<'p>{
	fn draw_in_render_window(&self, render_window:&mut RenderWindow) -> () {
		render_window.draw_sprite_rc(&self.body);
	}
	fn draw_in_render_texture(&self, render_texture:&mut RenderTexture) -> () {
		render_texture.draw_sprite_rc(&self.body);
	}
}