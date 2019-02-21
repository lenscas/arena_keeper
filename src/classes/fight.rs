
use crate::agents::character_agent::CharWithId;
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
	lethal_chance : i32
}
impl Fight{
	pub fn new(lethal_chance : i32, fighters : &Vec<CharacterId>) -> Self {
		Fight {
			time_left : 2,
			has_started : false,
			fighters : fighters.iter().map(|v| MaybeCharWithId::create_from_maybe(*v,None)).collect(),
			lethal_chance
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
				self.time_left = self.time_left - 1;
				if self.time_left <= 0 {
					return self.calc_outcome(&all);
				}
			}
		};
		FightOutcome {
			is_done : false,
			chars :self.fighters.clone(),
			earned_money : 0
		}
	}
	fn calculate_lost_life(&self, character : &Character, is_done : bool) -> Character {
		let hits_char1 = get_max(self.lethal_chance + 2 ) + 1;
		let mut total_life_lost = 0;
		for _ in 0..hits_char1 {
			total_life_lost = total_life_lost + get_max(self.lethal_chance + 5);
		};
		let mut new_char = character.clone();
		new_char.cur_health = new_char.cur_health - i64::from(total_life_lost);
		new_char.is_fighting= !is_done;
		if new_char.cur_health < 0 {
			new_char.cur_health = 0
		}
		new_char
	}
	fn calculate_money(&self) -> i64 {
		i64::from(get_max( (self.lethal_chance + 1) * 20))
	}
	fn calc_outcome(&self, characters : &Vec<CharWithId> ) -> FightOutcome {
		let  new_characters : Vec<MaybeCharWithId> = characters
			.iter()
			.map(
				|v|
				MaybeCharWithId::create(
					v.id,
					self.calculate_lost_life(&v.character, true)
				)
			)
			.collect();
		FightOutcome {
			is_done : true,
			chars : new_characters,
			earned_money : self.calculate_money()
		}
	}
	pub fn get_fighters_ids (&self) -> Vec<CharacterId> {
		self.fighters.iter().map(|v| v.id).collect()
	}
}