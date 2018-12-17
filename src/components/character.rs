
#[derive(PartialEq,Copy,Clone)]
pub enum CharacterTypes {
	Human,
	Merfolk
}
#[derive(PartialEq,Clone)]
pub struct Character {
	pub char_type : CharacterTypes,
	pub name : String,
	pub max_health: i64,
	pub cur_health : i64
}
impl Character {
	pub fn create_character(char_type : CharacterTypes ) -> Character {
		match char_type {
			CharacterTypes::Human => {
				Character {
					char_type : char_type,
					name : "Lumaceon".to_string(),
					max_health: 1,
					cur_health : 1
				}
			}
			CharacterTypes::Merfolk => {
				Character {
					char_type : char_type,
					name : "Tyler".to_string(),
					max_health: 5,
					cur_health : 2
				}
			}
		}
	}
	pub fn get_image(&self) -> String {
		match &self.char_type {
			CharacterTypes::Merfolk => {
				"/assets/images/merfolk.png".to_string()
			}
			CharacterTypes::Human => {
				"/assets/images/human.png".to_string()
			}
		}
	}
}