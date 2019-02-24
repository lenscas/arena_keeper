use crate::funcs::random::get_max;
use std::collections::HashMap;
use crate::classes::character::Character;
use indexmap::IndexMap;
use std::collections::HashSet;
use yew::prelude::worker::*;

use crate::agents::character_agent;
use crate::agents::clock_agent;

pub struct Worker {
	link: AgentLink<Worker>,
	component_list: HashSet<HandlerId>,
	char_worker: Box<Bridge<character_agent::Worker>>,
	money: i64,
	available : IndexMap< character_agent::CharacterId,Character>,
	current_id : u64,
	subbed_to_char_list : HashSet<HandlerId>,
	subbed_to_single_char :HashMap<character_agent::CharacterId,HashSet<HandlerId>>,
	_clock_worker: Box<Bridge<clock_agent::Worker>>,
	time_until_refil : i32
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
	SubtractAmount(i64),
	AddAmount(i64),
	BuyCharacter(character_agent::CharacterId),
	GetCharacter(character_agent::CharacterId),
	SwitchSubscribedCharacter(character_agent::CharacterId,character_agent::CharacterId),
	GetList,
}

impl Transferable for Request {}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
	NewAmount(i64),
	AnswerIdList(Vec<character_agent::CharacterId>),
	AnswerSingleChar(character_agent::MaybeCharWithId),
}

impl Transferable for Response {}

pub enum Msg {
	Res,
	Tick
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
		let clock_agent_callback = link.send_back(|_| Msg::Tick);
		let clock_worker = clock_agent::Worker::bridge(clock_agent_callback);
		let mut agent = Worker {
			link,
			component_list: HashSet::new(),
			money: 200,
			char_worker: character_worker,
			available: IndexMap::new(),
			current_id : 3,
			subbed_to_char_list : HashSet::new(),
			subbed_to_single_char : HashMap::new(),
			_clock_worker : clock_worker,
			time_until_refil: 5
		};
		agent.available.insert(character_agent::CharacterId { 0:1}, Character::create_character());
		agent.available.insert(character_agent::CharacterId { 0:2}, Character::create_character());
		agent.available.insert(character_agent::CharacterId { 0:3}, Character::create_character());
		agent
	}
	fn connected(&mut self, id: HandlerId) {
		self.component_list.insert(id);
		self.link.response(id, Response::NewAmount(self.money));
	}
	// Handle inner messages (of services of `send_back` callbacks)
	fn update(&mut self, msg: Self::Message) {
		if let Msg::Tick = msg {
			info!("In tick?");
			info!("{} {}",self.available.len(), self.time_until_refil);
			if self.available.len() < 3 && self.time_until_refil == 0  {
				info!("In if");
				self.current_id += 1;
				self.available.insert(character_agent::CharacterId { 0:1}, Character::create_character());
				self.time_until_refil = get_max(10);
				self.update_list_to_all();
			} else {
				self.time_until_refil -=1;
			}
		}
	}

	// Handle incoming messages from components of other agents.
	fn handle(&mut self, msg: Self::Input, who: HandlerId) {
		match msg {
			Request::AddAmount(to_add) => {
				self.money += to_add;
			},
			Request::SubtractAmount(to_subtract) => {
				self.money -= to_subtract;
			},
			Request::BuyCharacter(char_id) => {
				if let Some(character) = self.available.remove(&char_id) {
					if self.money < 100 {
						return;
					}
					self.money -= 100;
					self.char_worker.send(character_agent::Request::InsertCharacter(character));
					self.update_list_to_all();
				}
			},
			Request::GetCharacter(char_id) => {
                let m_chara = self.available.get(&char_id);
                if let Some(chara) = m_chara {
                    self.link.response(who, Response::AnswerSingleChar(character_agent::MaybeCharWithId::create_from_ref(char_id, chara)));
                    let map = self.subbed_to_single_char.entry(char_id).or_insert_with(HashSet::new);
                    map.insert(who);
                }
            },
			Request::GetList => {
				let as_vec = self.get_list_as_vec();
				self.link.response(who, Response::AnswerIdList(as_vec));
				self.subbed_to_char_list.insert(who);
			},
			Request::SwitchSubscribedCharacter(old_char_id,new_char_id) => {
				let sub_list = &mut self.subbed_to_single_char;
				let m_sub_list = sub_list.get_mut(&old_char_id);
				if let Some(sub_list) = m_sub_list {
					sub_list.remove(&who);
				}
				sub_list.entry(new_char_id.to_owned()).or_insert_with(HashSet::new).insert(who.to_owned());
				self.respond_with_single_char(who, new_char_id, &self.available);
			},
		};
		self.update_all();
	}
}
impl Worker {
	fn update_list_to_all(&self) {
		let as_vec = self.get_list_as_vec();
		for sub in &self.subbed_to_char_list {
			self.link.response(*sub, Response::AnswerIdList(as_vec.clone()));
		}
	}
	fn get_list_as_vec(&self) -> Vec<character_agent::CharacterId> {
		self.available
			.iter()
			.map( |v| v.0.to_owned() )
			.collect()
	}
	fn update_all(&mut self) {
		for v in self.component_list.iter() {
			self.link.response(v.to_owned(), Response::NewAmount(self.money));
		}
	}
	fn respond_with_single_char(&self, sub :HandlerId, char_id : character_agent::CharacterId, id_list : &IndexMap<character_agent::CharacterId,Character>) {
		let m_chara = id_list.get(&char_id);
		self.link.response(sub.to_owned(), Response::AnswerSingleChar( character_agent::MaybeCharWithId::create_from_maybe(char_id, m_chara.cloned())));
	}
}
