extern mod rsfml;
use rsfml::graphics::{FloatRect,RenderTexture,RenderWindow,Texture};
use rsfml::graphics::rc::{Sprite};
use rsfml::system::{Vector2f};
use rsfml::traits::Drawable;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GridBlock<'s>{
	block_body:Sprite,
	column:uint,
	row:uint,
	bounds:FloatRect,
}
// Methods for WallBlock
impl<'s> GridBlock<'s> {
	pub fn new(texture_loc:&str) -> GridBlock<'s> {
		let texture = match Texture::new_from_file(texture_loc){
			Some(texture)	=>	texture,
			None()			=>	fail!("Error, Wall texture"),
		};
		let texture_ref_cell = RefCell::new(texture);
		let texture_rc = Rc::new(texture_ref_cell);
		let sprite = match Sprite::new_with_texture(texture_rc){
			Some(sprite)	=>	sprite,
			None()			=>	fail!("Error, Wall sprite"),
		};
		let wall_bounds = sprite.get_global_bounds();
		GridBlock{block_body:sprite,column:0,row:0,bounds:wall_bounds}
	}
	pub fn get_bounds(&self)	-> FloatRect {
		self.block_body.get_global_bounds()
	}
	pub fn set_position(&mut self,column:uint,row:uint) {
		let column_f32 = column as f32; let row_f32 = row as f32;
		let real_position = Vector2f::new((column_f32 - 1.0) * 100.0, (row_f32 - 1.0)* 100.0);
		&self.block_body.set_position(&real_position);
	}
}
// Make WallBlock drawable
impl<'s> Drawable for GridBlock<'s>{
	fn draw_in_render_window(&self, render_window:&mut RenderWindow) -> () {
		render_window.draw_sprite_rc(&self.block_body);
	}
	fn draw_in_render_texture(&self, render_texture:&mut RenderTexture) -> () {
		render_texture.draw_sprite_rc(&self.block_body);
	}
}