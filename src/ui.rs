use rsfml::graphics::{Font,RenderTexture,RenderWindow};
use rsfml::graphics::rc::Text;
use rsfml::system::{Vector2f};
use rsfml::traits::Drawable;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Menu<'s> {
	body:Text,
}

impl <'s>Menu<'s> {
	pub fn new(content:&str) -> Menu {
		let font = match Font::new_from_file("../font/OstrichSans-Bold.otf") {
			Some(font)	=>	font,
			None()		=>	fail!(~"Error, menu font")
		};
		let ref_cell = RefCell::new(font);
		let rc_front = Rc::new(ref_cell);
		let text = match Text::new_init(content,rc_front,72){
			Some(text)	=>	text,
			None()		=>	fail!(~"Error, menu text")
		};
		Menu{body:text}
	}
	pub fn set_position(&mut self,window:&mut RenderWindow,x_pos:f32,y_pos:f32) {
		let window_size = window.get_size();
		let window_x = window_size.x as f32;
		let window_y = window_size.y as f32;
		let text_size = self.body.get_local_bounds();
		self.body.set_origin2f(text_size.width/2.0,text_size.height/2.0);
		self.body.set_position(&Vector2f::new(window_x*x_pos,window_y*y_pos))
	}
	pub fn set_text(&mut self,content:&str){
		self.body.set_string(content);
	}
}
impl<'s> Drawable for Menu<'s>{
	fn draw_in_render_window(&self, render_window:&mut RenderWindow) -> () {
		render_window.draw_text_rc(&self.body);
	}
	fn draw_in_render_texture(&self, render_texture:&mut RenderTexture) -> () {
		render_texture.draw_text_rc(&self.body);
	}
}