
use crate::agents::character_agent::CharacterId;


#[derive(PartialEq,Clone,Serialize, Deserialize,Debug)]
pub struct Fight {
	_time_left : i64,
	pub fighters : (CharacterId,CharacterId),
	pub has_started : bool,
	_lethal_chance : u64
}
impl Fight{
	pub fn new(lethal_chance : u64, fighters : (CharacterId, CharacterId)) -> Self {
		Fight {
			_time_left : 100,
			has_started : false,
			fighters : fighters,
			_lethal_chance : lethal_chance
		}
	}
	pub fn start(&mut self) {
		self.has_started = true;
	}
	pub fn get_name(&self) -> String {
		(self.fighters.0).0.to_string() + " " + &(self.fighters.1).0.to_string()
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
	/*
	fn _update(&mut self) -> bool {
		if self.has_started {
			self.time_left = self.time_left - 1;
			if self.time_left <= 0 {
				self.calc_outcome();
				true
			} else {
				false
			}
		} else {
			false
		}
	}

	fn calc_outcome(&mut self){
		//do stuff
	}
	*/
}