use crate::agents::character_agent::CharacterId;
use crate::agents::character_agent;
use crate::agents::money_agent;
use crate::agents::clock_agent;
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
	selected_fighters : Vec<CharacterId>,
	subbed_to_selected_fighters : HashMap<usize,HashSet<HandlerId>>,
	subbed_to_fight : HashMap<FightId,HashSet<HandlerId>>,
	subbed_to_fight_list : HashSet<HandlerId>,
	current_id : u64,
	char_worker: Box<Bridge<character_agent::Worker>>,
	money_worker: Box<Bridge<money_agent::Worker>>,
	_clock_worker: Box<Bridge<clock_agent::Worker>>,
	busy_fight: Option<FightId>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
	AddAsFighter(CharacterId),
	CreateFight(i32),
	GetAllFights,
	GetFight(FightId),
	GetReadyFighter(usize)
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
	Tick,
	Res,
	UpdateCharacter(character_agent::Response)
}

impl Agent for Worker {
	type Reach = Context; // Spawn only one instance per thread (all components could reach this)
	type Message = Msg;
	type Input = Request;
	type Output = Response;

	// Create an instance with a link to agent's environment.
	fn create(link: AgentLink<Self>) -> Self {
		let character_agent_callback = link.send_back(Msg::UpdateCharacter);
		let character_worker = character_agent::Worker::bridge(character_agent_callback);

		let money_agent_callback = link.send_back(|_| Msg::Res);
		let money_worker = money_agent::Worker::bridge(money_agent_callback);

		let clock_agent_callback = link.send_back(|_| Msg::Tick);
		let clock_worker = clock_agent::Worker::bridge(clock_agent_callback);
		Worker {
			link,
			component_list: HashSet::new(),
			fights: IndexMap::new(),
			current_id : 0,
			selected_fighters: Vec::new(),
			subbed_to_selected_fighters : HashMap::new(),
			subbed_to_fight : HashMap::new(),
			subbed_to_fight_list : HashSet::new(),
			char_worker: character_worker,
			money_worker,
			_clock_worker : clock_worker,
			busy_fight:None
		}
	}
	fn connected(&mut self, id: HandlerId) {
		self.component_list.insert(id);
	}
	// Handle inner messages (of services of `send_back` callbacks)
	fn update(&mut self, msg: Self::Message) {
		match msg {
			Msg::Tick =>{
				if let Some(fight_id) = &self.busy_fight {
					if let Some(fight) = self.fights.get_mut(fight_id) {
						let res = fight.update();
						if res.is_done {
							self.char_worker.send(character_agent::Request::UpdateMultipleCharacters(res.chars));
							self.fights.remove(fight_id);
							self.start_next(0);
							self.money_worker.send(money_agent::Request::AddAmount(res.earned_money));
							self.send_update_list();
						}
					}
				} else {
					self.start_next(1);
				}
			},
			Msg::UpdateCharacter(res) => {
				match res {
					character_agent::Response::AnswerMultipleChars(new_characters) => {
						for fighter in self.fights.iter_mut() {
							for new_character in new_characters.iter() {
								fighter.1.update_character(&new_character);
							}
						};
					},
					character_agent::Response::AnswerSingleChar(updated_character) => {
						for v in self.fights.iter_mut() {
							v.1.update_character(&updated_character);
						};
						if updated_character.character.is_none() {
							self.selected_fighters = self
								.selected_fighters
								.iter()
								.cloned()
								.filter(
									|v|
									*v == updated_character.id
								)
								.collect();
						};
					}
					_=> {
						unreachable!()
					}
				}
			}
			Msg::Res => {}
		}
	}

	// Handle incoming messages from components of other agents.
	fn handle(&mut self, msg: Self::Input, who: HandlerId) {
		match msg {
			Request::AddAsFighter(char_id) => {
				info!("Add fighter");
				let len = self.selected_fighters.len();
				if len > 2 {
					self.selected_fighters.remove(0);
				};
				self.selected_fighters.push(char_id);
				self.send_update_fighters(self.selected_fighters.iter().map(|v| Some(*v)).collect()) ;
			},
			Request::CreateFight(lethal_chance) => {
				if self.selected_fighters.len() >= 2 {
					let fight = Fight::new(lethal_chance, &self.selected_fighters);
					self.current_id += 1;
					let fight_id= FightId {0:self.current_id};
					self.fights.insert(fight_id, fight);
					if self.fights.len() == 1 {
						self.start_next(0);
					}
					for v in self.subbed_to_fight_list.iter() {
						self.link.response(*v, Response::UpdateFightList(self.create_fight_id_vec()))
					}
					self.char_worker.send(
						character_agent::Request::GetMultipleCharacters(
							self.selected_fighters.clone()
						)
					);

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
				self.subbed_to_selected_fighters.entry(i).or_default().insert(who.to_owned());
				let char_id = self.selected_fighters.get(i);
				self.link.response(who,Response::UpdateFighter(char_id.cloned()));
			}

		};
	}
}
impl Worker {
	fn send_update_fighters(&mut self, mut fighters : Vec<Option<CharacterId>> ) {
		if fighters.len() < 2 {
			fighters.push(None);
		};
		for fighter_id in fighters.iter().enumerate() {
			for sub in self.subbed_to_selected_fighters.entry(fighter_id.0).or_default().iter() {
				info!("Send update");
				self.link.response(*sub, Response::UpdateFighter(*fighter_id.1));
			}
		}
	}

	fn create_fight_id_vec(&self) -> Vec<FightId> {
		self.fights.iter()
			.map(
				|v| v.0.to_owned()
			)
			.collect::<Vec<FightId>>()
	}
	fn send_update_list(&self) {
		for v in &self.subbed_to_fight_list {
			self.link.response(*v, Response::UpdateFightList(self.create_fight_id_vec()));
		}
	}
	fn start_next(&mut self, at: usize) {
		let m_next = self.fights.iter_mut().nth(at);
		if let Some(next) = m_next {
			self.busy_fight = Some(*next.0);
			next.1.start();
			let fighters = next.1.get_fighters_ids();
			for fighter in fighters.iter() {
				self.char_worker.send(character_agent::Request::SetCharacterAsFighting(*fighter));
			}
		};
	}
}
