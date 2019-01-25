use core::fmt;
use stdweb::Value;
use stdweb::unstable::TryInto;

#[derive(PartialEq,Copy,Clone,Debug,Serialize,Deserialize)]
pub enum CharacterTypes {
	Human,
	Merfolk
}
impl fmt::Display for CharacterTypes {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

#[derive(PartialEq,Clone,Serialize, Deserialize,Debug)]
pub struct Character {
	pub char_type : CharacterTypes,
	pub name : String,
	pub max_health: i64,
	pub cur_health : i64
}
impl Character {
	fn create_name(char_type : CharacterTypes) -> String {
		let as_str = char_type.to_string();
		let v: Value = js! {
			return geneateName(@{as_str})
		};
		let v : String = v.try_into().expect("Something wend wrong");
		v
	}
	pub fn create_character(char_type : CharacterTypes ) -> Character {
		let name = Character::create_name(char_type);
		match char_type {
			CharacterTypes::Human => {
				Character {
					char_type : char_type,
					name : name,
					max_health: 1,
					cur_health : 1
				}
			}
			CharacterTypes::Merfolk => {
				Character {
					char_type : char_type,
					name : name,
					max_health: 5,
					cur_health : 2
				}
			}
		}
	}
	pub fn get_image(&self) -> (String,String){
		(
			String::from("/assets/images/")+&self.char_type.to_string() +".png",
			self.char_type.to_string()
		)
	}
}