use crate::agents::character_agent::CharacterId;
use std::collections::HashMap;
use std::collections::HashSet;
use indexmap::IndexMap;
use yew::prelude::worker::*;

use crate::classes::fight::Fight;

#[derive(Copy,Clone, PartialEq, Eq, Hash,Serialize, Deserialize, Debug)]
pub struct FightId(pub u64);

pub struct Worker {
	link: AgentLink<Worker>,
	component_list: HashSet<HandlerId>,
	fights: IndexMap<FightId,Fight>,
	selected_fighters : (Option<CharacterId>,Option<CharacterId>),
	subbed_to_selected_fighters : HashMap<u8,HashSet<HandlerId>>,
	subbed_to_fight : HashMap<FightId,HashSet<HandlerId>>,
	subbed_to_fight_list : HashSet<HandlerId>,
	current_id : u64
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
	UpdateFighter(Option<CharacterId>),
	UpdateFight(Fight),
	UpdateFightList(Vec<FightId>)
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
			fights: IndexMap::new(),
			current_id : 0,
			selected_fighters: (None,None),
			subbed_to_selected_fighters : HashMap::new(),
			subbed_to_fight : HashMap::new(),
			subbed_to_fight_list : HashSet::new()
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
			Request::CreateFight(lethal_chance) => {
				if let Some(fighter1) = self.selected_fighters.0 {
					if let Some(fighter2) = self.selected_fighters.1 {
						let mut fight = Fight::new(lethal_chance, (fighter1,fighter2));
						self.current_id = self.current_id + 1;
						if self.fights.len() == 0 {
							fight.start();
						}
						self.fights.insert(FightId {0:self.current_id}, fight);
						for v in self.subbed_to_fight_list.iter() {
							self.link.response(*v, Response::UpdateFightList(self.create_fight_id_vec()))
						}
					}
				}
			},
			Request::GetFight(fight_id) => {
				let maybe_fight = self.fights.get(&fight_id);
				if let Some(fight) = maybe_fight {
					self.link.response(who, Response::UpdateFight(fight.to_owned()));
					self.subbed_to_fight.entry(fight_id).or_default().insert(who);
				}
			},
			Request::GetAllFights => {
				self.link.response(
					who,
					Response::UpdateFightList(self.create_fight_id_vec())
				);
				self.subbed_to_fight_list.insert(who);
			},
			Request::GetReadyFighter(i) => {
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
		for v in self.subbed_to_selected_fighters.entry(side).or_default().iter() {
			self.link.response(*v, Response::UpdateFighter(fighter));
		};
	}
	fn create_fight_id_vec(&self) -> Vec<FightId> {
		self.fights.iter()
			.map(
				|v| v.0.to_owned()
			)
			.collect::<Vec<FightId>>()
	}
}
