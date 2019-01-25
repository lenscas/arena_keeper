use std::collections::HashSet;
use yew::prelude::worker::*;

use crate::agents::character_agent;

pub struct Worker {
	link: AgentLink<Worker>,
	component_list: HashSet<HandlerId>,
	char_worker: Box<Bridge<character_agent::Worker>>,
	money: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
	SubtractAmount(i64),
	AddAmount(i64),
	BuyCharacter(character_agent::CharacterId)
}

impl Transferable for Request {}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
	NewAmount(i64),
}

impl Transferable for Response {}

pub enum Msg {
	Res
}

impl Agent for Worker {
	// Available:
	// - `Job` (one per bridge)
	// - `Context` (shared in the same thread)
	// - `Public` (separate thread).
	type Reach = Context; // Spawn only one instance per thread (all components could reach this)
	type Message = Msg;
	type Input = Request;
	type Output = Response;

	// Create an instance with a link to agent's environment.
	fn create(link: AgentLink<Self>) -> Self {
		let character_agent_callback = link.send_back(|_| Msg::Res);
		let character_worker = character_agent::Worker::bridge(character_agent_callback);
		Worker {
			link,
			component_list: HashSet::new(),
			money: 200,
			char_worker: character_worker
		}
	}
	fn connected(&mut self, id: HandlerId) {
		self.component_list.insert(id);
		self.link.response(id, Response::NewAmount(self.money));
	}
	// Handle inner messages (of services of `send_back` callbacks)
	fn update(&mut self, _msg: Self::Message) {}

	// Handle incoming messages from components of other agents.
	fn handle(&mut self, msg: Self::Input, _who: HandlerId) {
		match msg {
			Request::AddAmount(to_add) => {
				self.money = self.money + to_add;
			},
			Request::SubtractAmount(to_subtract) => {
				self.money = self.money - to_subtract;
			},
			Request::BuyCharacter(char_id) => {
				if self.money < 100 {
					return;
				}
				self.money = self.money - 100;
				self.char_worker.send(character_agent::Request::BuyCharacter(char_id));
			}
		};
		self.update_all();
	}
}
impl Worker {
	fn update_all(&mut self) {
		for v in self.component_list.iter() {
			 self.link.response(v.to_owned(), Response::NewAmount(self.money));
		}
	}
}
