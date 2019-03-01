
use std::collections::HashMap;
use crate::funcs::array_helper::ensure_all_some_chars;
use crate::funcs::random::get_max;
use crate::classes::character::Character;
use crate::classes::fight_outcome::FightOutcome;
use crate::agents::character_agent::CharacterId;
use crate::agents::character_agent::MaybeCharWithId;

#[derive(PartialEq,Clone,Serialize, Deserialize,Debug)]
pub struct Fight {
	time_left : i64,
	pub fighters : Vec<MaybeCharWithId>,
	pub has_started : bool,
	lethal_chance : i32,
	turns_taken : i32,
	spectacle_level : i32
}
impl Fight{
	pub fn new(lethal_chance : i32, fighters : &[CharacterId]) -> Self {
		Fight {
			time_left : 2,
			has_started : false,
			fighters : fighters.iter().map(|v| MaybeCharWithId::create_from_maybe(*v,None)).collect(),
			lethal_chance,
			turns_taken : 0,
			spectacle_level:0
		}
	}
	pub fn start(&mut self) -> bool{
		self.has_started = true;
		self.has_started
	}
	pub fn update_character(&mut self, character: &MaybeCharWithId) {
		self.fighters
			.iter_mut()
			.for_each(
				|v|
				if v.id == character.id {
					*v = character.to_owned();
				}
			);
	}
	pub fn get_started_text(&self) ->String {
		String::from(
			if self.has_started {
				"Busy"
			} else {
				"Waiting"
			}
		)
	}
	pub fn update(&mut self) -> FightOutcome {
		if self.has_started {
			let m_all = ensure_all_some_chars( &self.fighters);

			if let Some(all) = m_all {
				let mut outcomes : HashMap<CharacterId,Character> = all.iter().cloned().map(|v| (v.id,v.character)).collect();
				self.turns_taken +=1;
				for action in all.iter()
					.map(
						|v| v.character.make_action(&all)
					)
				{
					let on = all.iter().find(|v| v.id==action.on).unwrap();
					let rolled_agil = get_max(on.character.agility*2);
					let rolled_acc = get_max(action.accuracy * 2 + 2);
					if rolled_agil <= rolled_acc {
						let diff = rolled_agil - rolled_acc;
						let bonus = if diff > 20 {
							10
						} else if diff > 10 {
							5
						} else {
							0
						};
						let rolled_def = get_max(on.character.armor * 2);
						let rolled_str = get_max(action.strength * 2 + bonus + 2);
						if rolled_def <= rolled_str {
							let diff = rolled_def - rolled_str;
							let bonus = if diff > 20 {
								10
							} else if diff > 10 {
								5
							} else {
								0
							};
							let in_outcomes = outcomes.entry(on.id).or_insert_with(|| on.character.to_owned());
							in_outcomes.cur_health -= get_max(action.damage + bonus);
							in_outcomes.is_fighting = in_outcomes.cur_health > (20-self.lethal_chance) *80 /20;
							self.spectacle_level += action.spectacle_delta;
						}
					}
				}
				let is_done = outcomes.iter().fold(0,|val,character| if character.1.is_fighting {val} else {val+1}) >= self.fighters.len() - 1;
				let earned_money = if is_done {
					self.calculate_money()
				} else {
					0
				};

				return FightOutcome {
					is_done,
					chars : outcomes
						.iter()
						.map(
							|v|
								MaybeCharWithId::create(
									*v.0,
									if is_done {
										let mut t = v.1.clone();
										t.is_fighting = false;
										t
									} else {
										v.1.clone()
									}
								)
						)
						.collect(),
					earned_money
				}
			}
		};
		FightOutcome {
			is_done : false,
			chars :self.fighters.clone(),
			earned_money : 0
		}
	}
	fn calculate_money(&self) -> i64 {
		let mut gained_money = get_max( (self.lethal_chance + 1) * 20);
		if self.turns_taken < 5 {
			gained_money -= get_max((self.lethal_chance + 1) * 10);
		}
		gained_money += match self.spectacle_level {
			-10 ... -5 => -20,
			-4  ...  0 => -10,
			1   ...  5 =>  10,
			6   ... 10 =>  20,
			count      => {
				if count < -20 {
					-30
				} else {
					30
				}
			}
		};
		i64::from(gained_money)
	}
	pub fn get_fighters_ids (&self) -> Vec<CharacterId> {
		self.fighters.iter().map(|v| v.id).collect()
	}
}