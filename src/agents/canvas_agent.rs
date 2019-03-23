use stdweb::traits::*;
use stdweb::web::document;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::event::{
	ConcreteEvent,
	KeyDownEvent,
	KeyUpEvent,
	IMouseEvent,
	MouseDownEvent,
	MouseMoveEvent,
	MouseUpEvent,
	MouseLeaveEvent,
	MouseEnterEvent

};
use stdweb::unstable::TryInto;
use std::collections::HashSet;
use yew::prelude::worker::*;
use yew::Callback;

//first we define a VERY basic service that will be used by the agent
//this service adds an event to the DOM that will listen to keyboard events.
#[derive(Default)]
struct CanvasAgent {}
impl CanvasAgent {
	pub fn new() -> Self {
		CanvasAgent {}
	}
	pub fn on_down(&mut self, callback: Callback<KeyDownEvent>) {
		document()
			.body()
			.unwrap()
			.add_event_listener::<KeyDownEvent, _>(move |e| callback.emit(e));
	}
	pub fn on_up(&mut self, callback: Callback<KeyUpEvent>) {
		document()
			.body()
			.unwrap()
			.add_event_listener::<KeyUpEvent, _>(move |e| callback.emit(e));
	}
	pub fn canvas<T: 'static>(&mut self, callback : Callback<T>) where T : ConcreteEvent {
		let document = document();
		let canvas : CanvasElement = document.query_selector( "#canvas" ).unwrap().unwrap().try_into().unwrap();
		canvas.add_event_listener::<T,_>(move |e| callback.emit(e));
	}
}

pub struct Worker {
	link: AgentLink<Worker>,
	component_list: HashSet<HandlerId>,
	pressed_keys: HashSet<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Request {}

impl Transferable for Request {}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Response {
	//happens when a key is pressed/stays pressed
	KeyDown(String),
	//happens when a key is released
	KeyUp(String),
	//happens when a key is pressed and ONLY triggers again after the key has been released.
	KeyType(String),
	MouseMove(f64,f64),
	MouseUp,
	MouseDown,
	MouseLeave,
	MouseEnter

}

impl Transferable for Response {}

pub enum Msg {
	//happens when a key is pressed/stays pressed
	KeyDown(KeyDownEvent),
	//happens when a key is released
	KeyUp(KeyUpEvent),
	MouseDown(MouseDownEvent),
	MouseUp(MouseUpEvent),
	MouseLeave(MouseLeaveEvent),
	MouseEnter(MouseEnterEvent),
	MouseMove(MouseMoveEvent),
}

impl Agent for Worker {
	type Reach = Context;
	type Message = Msg;
	type Input = Request;
	type Output = Response;

	fn create(link: AgentLink<Self>) -> Self {
		let mut keyboard = CanvasAgent::new();
		let on_down = link.send_back(Msg::KeyDown);
		let on_up = link.send_back(Msg::KeyUp);
		keyboard.on_down(on_down);
		keyboard.on_up(on_up);
		keyboard.canvas(link.send_back(Msg::MouseDown));
		keyboard.canvas(link.send_back(Msg::MouseUp));
		keyboard.canvas(link.send_back(Msg::MouseEnter));
		keyboard.canvas(link.send_back(Msg::MouseLeave));
		keyboard.canvas(link.send_back(Msg::MouseMove));

		Worker {
			link,
			component_list: HashSet::new(),
			pressed_keys: HashSet::new(),
		}
	}
	fn connected(&mut self, id: HandlerId) {
		self.component_list.insert(id);
	}
	// Handle inner messages (of services of `send_back` callbacks)
	fn update(&mut self, msg: Self::Message) {
		match msg {
			Msg::KeyDown(down) => {
				let key = down.code();
				if !self.pressed_keys.contains(&key) {
					self.send_event(Response::KeyType(key.clone()));
					self.pressed_keys.insert(key.clone());
				}
				self.send_event(Response::KeyDown(key));
			}
			Msg::KeyUp(up) => {
				let key = up.code();
				self.pressed_keys.remove(&key);
				self.send_event(Response::KeyUp(key));
			},
			Msg::MouseDown(_) => self.send_event(Response::MouseDown),
			Msg::MouseUp(_) => self.send_event(Response::MouseUp),
			Msg::MouseMove(mouse) => self.send_event(Response::MouseMove(
				mouse.offset_x(),
				mouse.offset_y()
			)),
			Msg::MouseLeave(_) => self.send_event(Response::MouseLeave),
			Msg::MouseEnter(_) => self.send_event(Response::MouseEnter),
		}
	}

	// Handle incoming messages from components of other agents.
	fn handle(&mut self, _: Self::Input, _: HandlerId) {}
}
impl Worker {
	fn send_event(&self, event: Response) {
		for sub in &self.component_list {
			self.link.response(*sub, event.clone());
		}
	}
}
