use crate::agents::character_agent::CharacterId;
use std::collections::HashMap;
use std::collections::HashSet;
use yew::prelude::worker::*;

use crate::classes::fight::Fight;

#[derive(Copy,Clone, PartialEq, Eq, Hash,Serialize, Deserialize, Debug)]
pub struct FightId(u64);

pub struct Worker {
	link: AgentLink<Worker>,
	component_list: HashSet<HandlerId>,
	_fights: HashMap<FightId,Fight>,
	selected_fighters : (Option<CharacterId>,Option<CharacterId>),
	subbed_to_selected_fighters : HashMap<u8,HashSet<HandlerId>>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
	AddAsFighter(CharacterId),
	CreateFight(u64),
	GetAllFights,
	GetFight(FightId),
	GetReadyFighter(u8)
}

impl Transferable for Request {}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
	UpdateFighter(Option<CharacterId>)
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
		Worker {
			link,
			component_list: HashSet::new(),
			_fights: HashMap::new(),
			selected_fighters: (None,None),
			subbed_to_selected_fighters : HashMap::new()
		}
	}
	fn connected(&mut self, id: HandlerId) {
		self.component_list.insert(id);
	}
	// Handle inner messages (of services of `send_back` callbacks)
	fn update(&mut self, _msg: Self::Message) {

	}

	// Handle incoming messages from components of other agents.
	fn handle(&mut self, msg: Self::Input, who: HandlerId) {
		match msg {
			Request::AddAsFighter(char_id) => {
				if self.selected_fighters.0.is_none() {
					self.selected_fighters.0 = Some(char_id);
					self.send_update_fighter(0);
				} else if self.selected_fighters.1.is_none() {
					self.selected_fighters.1 = Some(char_id);
					self.send_update_fighter(1);
				} else {
					let old_char = self.selected_fighters.1.clone();
					self.selected_fighters.0 = old_char;
					self.selected_fighters.1 = Some(char_id);
					self.send_update_fighter(0);
					self.send_update_fighter(1);
				}
			},
			Request::CreateFight(_lethal_chance) => {
			},
			Request::GetFight(_fight) => {

			},
			Request::GetAllFights => {

			},
			Request::GetReadyFighter(i) => {
				info!("{}", i);
				let m_char_id = if i == 0 {
					self.selected_fighters.0
				} else {
					self.selected_fighters.1
				};
				self.subbed_to_selected_fighters.entry(i).or_default().insert(who.to_owned());
				self.link.response(who,Response::UpdateFighter(m_char_id.clone()));
			}

		};
	}
}
impl Worker {
	fn send_update_fighter(&mut self, side : u8){
		let fighter = if side == 0 {
			self.selected_fighters.0
		} else {
			self.selected_fighters.1
		};
		for v in self.subbed_to_selected_fighters.entry(side).or_default().iter(){

			self.link.response(*v, Response::UpdateFighter(fighter));
		};
	}
}
