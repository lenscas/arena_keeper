use stdweb::web::html_element::{
	CanvasElement,
	ImageElement
};
use yew::prelude::*;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    CanvasRenderingContext2d
};

use crate::agents::game_agent;
use crate::agents::canvas_agent;
use crate::config::GRID;
pub struct Game {
	_clock_worker: Box<Bridge<game_agent::Worker>>,
	_canvas_worker : Box<Bridge<canvas_agent::Worker>>,
	context : CanvasRenderingContext2d,
	canvas : CanvasElement,
	blocks : Vec<(f64,f64)>,
	render_block : bool,
	block_x : f64,
	block_y : f64

}

pub enum Msg {
	Tick(game_agent::Response),
	Key(canvas_agent::Response)
}

#[derive(PartialEq, Clone)]
pub struct Props {}

impl Default for Props {
	fn default() -> Self {
		Props {
		}
	}
}

impl Component for Game {
	type Message = Msg;
	type Properties = Props;
	fn create(_: Self::Properties, mut link : ComponentLink<Self>) -> Self {
		let document = document();
		let canvas: CanvasElement = document.query_selector( "#canvas" ).unwrap().unwrap().try_into().unwrap();
		let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
		let clock_agent_callback = link.send_back(Msg::Tick);
		let clock_worker = game_agent::Worker::bridge(clock_agent_callback);
		let canvas_agent_callback = link.send_back(Msg::Key);
		let canvas_worker = canvas_agent::Worker::bridge(canvas_agent_callback);
		Game {
			context,
			canvas,
			blocks : Vec::new(),
			_clock_worker : clock_worker,
			_canvas_worker : canvas_worker,
			render_block : false,
			block_x : 0.0,
			block_y : 0.0

		}
	}
	fn update(&mut self, msg : Self::Message) -> ShouldRender {
		match msg {
			Msg::Tick(res) => {
				match res {
					game_agent::Response::NewPos(update) => {
						self.blocks = update;
						true
					}
				}
			},
			Msg::Key(kind)=> {
				match kind {
					canvas_agent::Response::KeyDown(key) => {
						match key.as_ref() {
							"ArrowRight" => {
								self.context.translate(1.0, 0.0);
							},
							"ArrowLeft" => {
								self.context.translate(-1.0, 0.0);
							},
							"ArrowDown" => {
								self.context.translate(0.0, 1.0);
							},
							"ArrowUp"   => {
								self.context.translate(0.0, -1.0);
							},
							_ =>()
						};
						info!("{}",key);
						true
					},
					canvas_agent::Response::MouseDown => {
						self.render_block = true;
						true
					},
					canvas_agent::Response::MouseUp => {
						self.render_block = false;
						true
					},
					canvas_agent::Response::MouseMove(x,y) => {
						self.block_x = x / f64::from(self.canvas.offset_width()) * f64::from(self.canvas.width());
						self.block_y = y / f64::from( self.canvas.offset_height()) * f64::from(self.canvas.height());
						self.render_block
					},
					canvas_agent::Response::MouseLeave => {
						self.render_block = false;
						true
					},
					canvas_agent::Response::MouseEnter => false,
					_ => {
						//info!("{}",default);
						info!("In other event?");
						false
					}
				}
			}
		}
	}
	fn change(&mut self, _ : Self::Properties) -> ShouldRender {
		true
	}
}

impl Renderable<Game> for Game {
	fn view(&self) -> Html<Self> {
		let canvas = self.context.get_canvas();

		self.context.clear_rect(0.0,0.0, f64::from(canvas.width()) ,f64::from(canvas.height()));
		let image = ImageElement::new();
		image.set_src("/assets/tiles/borders.png");
		let res = image.set_attribute("style", "height:10px;width:10px");
		if res.is_err() {
			return html!{<></>}
		}
		for x in (0 .. self.canvas.width()).step_by(GRID.as_usize()) {
			let x = f64::from(x);
			for y in (0..self.canvas.height()).step_by(GRID.as_usize()) {
				let fsize = GRID.as_float();
				let res = self.context.draw_image_d(image.clone(), x, f64::from(y), fsize, fsize);
				if res.is_err() {
					return html!{<></>}
				}
			}
		}
		for square in &self.blocks {
			self.context.fill_rect(square.0, square.1, 10.0,10.0);
		}

		if self.render_block {
			self.context.set_stroke_style_color("#ff0000");
			self.context.set_fill_style_color("#ff0000");
			let coords_in_grid = GRID.snap_to_grid(self.block_x,self.block_y);
			self.context.fill_rect(coords_in_grid.0 ,coords_in_grid.1, 13.0,13.0);
			self.context.set_stroke_style_color("#000000");
			self.context.set_fill_style_color("#000000");
		};
		html!{<></>}
	}
}
