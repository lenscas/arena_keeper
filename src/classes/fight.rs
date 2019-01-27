
use crate::funcs::random::getMax;
use crate::classes::character::Character;
use crate::classes::fight_outcome::FightOutcome;
use crate::agents::character_agent::CharacterId;


#[derive(PartialEq,Clone,Serialize, Deserialize,Debug)]
pub struct Fight {
	time_left : i64,
	pub fighters : (
		(CharacterId, Option<Character>),
		(CharacterId, Option<Character>)
	),
	pub has_started : bool,
	lethal_chance : i32
}
impl Fight{
	pub fn new(lethal_chance : i32, fighters : (CharacterId, CharacterId)) -> Self {
		Fight {
			time_left : 2,
			has_started : false,
			fighters : (
				(fighters.0,None),
				(fighters.1,None)
			),
			lethal_chance
		}
	}
	pub fn start(&mut self) {
		self.has_started = true;
	}
	pub fn update_character(&mut self, char_id : &CharacterId, character: &Character){
		if (self.fighters.0).0 == *char_id {
			(self.fighters.0).1 = Some(character.to_owned());
		} else if (self.fighters.1).0 == *char_id {
			(self.fighters.1).1 = Some(character.to_owned());
		}
	}
	/*
	pub fn get_name(&self) -> String {
		(self.fighters.0).0.to_string() + " " + &(self.fighters.1).0.to_string()
	}*/
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
			if let Some(char1) = &(self.fighters.0).1 {
				if let Some(char2) = &(self.fighters.1).1 {
					self.time_left = self.time_left - 1;
					if self.time_left <= 0 {
						info!("in update");
						return self.calc_outcome(
							((self.fighters.0).0,char1),
							((self.fighters.1).0,char2),
						);
					}
				}
			}
		} else {
			info!("Why did it get here?");
		};
		FightOutcome {
			is_done : false,
			chars : (
				(
					(self.fighters.0).0,
					Character::create_character()
				),
				(
					(self.fighters.1).0,
					Character::create_character()
				)
			),
			earned_money : 0
		}
	}
	fn calculate_lost_life(&self, character : &Character) -> Character {
		let hits_char1 = getMax(self.lethal_chance + 2 ) + 1;
		let mut total_life_lost = 0;
		for _ in 0..hits_char1 {
			total_life_lost = total_life_lost + getMax(self.lethal_chance + 5);
		};
		let mut new_char = character.clone();
		new_char.cur_health = new_char.cur_health - i64::from(total_life_lost);
		if new_char.cur_health < 0 {
			new_char.cur_health = 0
		}
		new_char
	}
	fn calculate_money(&self) -> i64 {
		i64::from(getMax( (self.lethal_chance + 1) * 20))
	}
	fn calc_outcome(&self, char1 : (CharacterId,&Character), char2: (CharacterId,&Character)) ->FightOutcome {
		let new_char_1 = self.calculate_lost_life(char1.1);
		let new_char_2 = self.calculate_lost_life(char2.1);
		FightOutcome {
			is_done : true,
			chars : (
				(
					char1.0,
					new_char_1
				),
				(
					char2.0,
					new_char_2
				)
			),
			earned_money : self.calculate_money()
		}
	}
	pub fn get_fighters_ids (&self) -> (CharacterId,CharacterId) {
		(
			(self.fighters.0).0,
			(self.fighters.1).0,
		)
	}
}