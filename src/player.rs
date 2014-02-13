extern mod rsfml;
use rsfml::graphics::{FloatRect,RenderTexture,RenderWindow,Texture};
use rsfml::graphics::rc::{Sprite};
use rsfml::system::Vector2f;
use rsfml::traits::Drawable;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Avatar<'p>{
	body:Sprite,
	position:Vector2f,
	bounds:FloatRect,
	force:Vector2f,
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
		let avatar_bounds = player_sprite.get_global_bounds();
		Avatar{
			body:player_sprite,
			position:Vector2f::new(0.0,0.0),
			bounds:avatar_bounds,
			force:Vector2f::new(0.0,10.0),
		}
	}
	pub fn get_bounds(&self)	-> FloatRect {
		self.body.get_global_bounds()
	}
	pub fn get_position(&self) -> Vector2f {
		self.position
	}
	pub fn set_position(&mut self, new_position:Vector2f){
		self.position = new_position;
		self.body.set_position(&new_position)
	}
	pub fn walk(&mut self, shift:f32){
		self.position.x += shift;
		self.set_position(self.position);
	}
	pub fn jump(&mut self, shift:f32){
		self.position.y += shift;
		self.set_position(self.position);
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
impl<'p> Clone for Avatar<'p> {
	fn clone(&self) -> Avatar{
		self.clone()
	}
}