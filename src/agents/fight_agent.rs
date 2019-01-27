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
	selected_fighters : (Option<CharacterId>,Option<CharacterId>),
	subbed_to_selected_fighters : HashMap<u8,HashSet<HandlerId>>,
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
	Tick,
	Res,
	UpdateCharacter(character_agent::Response)
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
		let character_agent_callback = link.send_back(|res| Msg::UpdateCharacter(res));
		let character_worker = character_agent::Worker::bridge(character_agent_callback);

		let money_agent_callback = link.send_back(|_| Msg::Res);
		let money_worker = money_agent::Worker::bridge(money_agent_callback);

		let clock_agent_callback = link.send_back(|_| Msg::Tick);
		let clock_worker = clock_agent::Worker::bridge(clock_agent_callback);
		info!{"We should have a clock?"}
		Worker {
			link,
			component_list: HashSet::new(),
			fights: IndexMap::new(),
			current_id : 0,
			selected_fighters: (None,None),
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
							self.char_worker.send(character_agent::Request::UpdateCharacter( (res.chars.0).0,(res.chars.0).1));
							self.char_worker.send(character_agent::Request::UpdateCharacter( (res.chars.1).0,(res.chars.1).1));
							self.fights.remove(fight_id);
							info!("len: {}",self.fights.len());
							let m_next = self.fights.iter_mut().nth(0);
							if let Some(next) = m_next {
								info!("in if");
								self.busy_fight = Some(*next.0);
								info!("id: {}", (next.0).0);
								next.1.start();

							};
							info!("len2: {}",self.fights.len());
							self.money_worker.send(money_agent::Request::AddAmount(res.earned_money));
							self.send_update_list();
						}
					}
				} else {
					let m_next = self.fights.iter_mut().nth(1);
					if let Some(next) = m_next {
						self.busy_fight = Some(*next.0);
						next.1.start();
					};
				}
			},
			Msg::UpdateCharacter(res) => {
				match res {
					character_agent::Response::AnswerDoubleChar(char1,char2) => {
						for v in self.fights.iter_mut() {
							v.1.update_character(&char1.0,&char1.1);
							v.1.update_character(&char2.0,&char2.1);
						};
					},
					character_agent::Response::AnswerSingleChar(character,id) => {
						for v in self.fights.iter_mut() {
							v.1.update_character(&id,&character);
						};
						if let Some(selected1) = self.selected_fighters.0 {
							info!("{}", (selected1).0);
							if selected1 == id && character.cur_health == 0 {
								self.selected_fighters.0 = None;
								info!("in this");
								self.send_update_fighter(0);
							}
						};
						if let Some(selected2) = self.selected_fighters.1 {
							info!("{}", (selected2).0);
							if selected2 == id && character.cur_health == 0 {
								self.selected_fighters.1 = None;
								info!("in this");
								self.send_update_fighter(1);
							}
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
						let fight_id= FightId {0:self.current_id};
						if self.fights.len() == 0 {
							fight.start();
							self.busy_fight = Some(fight_id);
						}

						self.fights.insert(fight_id, fight);

						for v in self.subbed_to_fight_list.iter() {
							self.link.response(*v, Response::UpdateFightList(self.create_fight_id_vec()))
						}
						self.char_worker.send(character_agent::Request::GetDoubleCharacter((fighter1,fighter2)));
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
	fn send_update_list(&self) {
		for v in &self.subbed_to_fight_list {
			self.link.response(*v, Response::UpdateFightList(self.create_fight_id_vec()));
		}
	}
}
