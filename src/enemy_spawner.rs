extern mod rsfml;
use rsfml::graphics::{RenderTexture,RenderWindow};
use rsfml::graphics::rc::{RectangleShape};
use rsfml::system::Vector2f;
use rsfml::traits::Drawable;
use player::Avatar;

pub struct MobSpawn<'s> {
	body:RectangleShape,
	mob:Avatar<'s>,
	column:uint,
	row:uint,
	is_ready:bool,
}

impl<'s> MobSpawn<'s> {
	pub fn new() -> MobSpawn {
		let blank = match RectangleShape::new() {
			Some(blank)	=>	blank,
			None()		=>	fail!(~"Error, Mobspawn blank creaion")
		};
		let mut enemy = Avatar::new("../img/enemy.png");
		enemy.force = Vector2f::new(-2.0,10.0);
		MobSpawn{body:blank,mob:enemy,column:0,row:0,is_ready:true}
	}
	pub fn get_position(&self) -> Vector2f {
		self.body.get_position()
	}
	pub fn set_position(&mut self,column:uint,row:uint) {
		let column_f32 = column as f32; let row_f32 = row as f32;
		let real_position = Vector2f::new((column_f32 - 1.0) * 100.0, (row_f32 - 1.0)* 100.0);
		&self.mob.set_position(real_position);
		&self.body.set_position(&real_position);
	}
}
impl<'s> Drawable for MobSpawn<'s>{
	fn draw_in_render_window(&self, render_window:&mut RenderWindow) -> () {
		render_window.draw_rectangle_shape_rc(&self.body);
	}
	fn draw_in_render_texture(&self, render_texture:&mut RenderTexture) -> () {
		render_texture.draw_rectangle_shape_rc(&self.body);
	}
}