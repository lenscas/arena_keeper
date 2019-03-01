use crate::funcs::random::get_max;
use crate::agents::character_agent::CharWithId;
use crate::funcs::random::get_between;
use crate::generated::create_description::generate_description;
use crate::generated::create_name::generate_name;
use crate::generated::species_types::SpeciesTypes;
use crate::generated::generate_type::generate_type;
use crate::generated::generate_image::generate_image;
use crate::classes::action::Action;

#[derive(Eq,Hash,PartialEq,Clone,Serialize, Deserialize,Debug)]
pub struct Character {
	pub char_type : SpeciesTypes,
	pub name : String,
	pub max_health: i32,
	pub cur_health : i32,
	pub image:String,
	pub description : String,
	pub is_fighting : bool,
	pub fan_amount : i32,
	pub agility : i32,
	pub armor : i32,
	pub strength : i32,
	pub accuracy : i32
}
impl Character {
	pub fn create_character() -> Character {
		let species = generate_type();
		let name = generate_name(species);
		let image = generate_image(species);
		let description = generate_description(species);
		let health = get_between(90,110);
		Character {
			char_type : species,
			name,
			max_health : health,
			cur_health : get_between(health-10,health),
			image,
			description,
			is_fighting : false,
			fan_amount : get_between(-5,5),
			agility : get_between(0,10),
			armor : get_between(0,10),
			strength : get_between(0,10),
			accuracy : get_between(0,10)
		}
	}
	pub fn update(&mut self) -> bool {
		if self.cur_health < self.max_health && self.cur_health > 0 && !self.is_fighting {
			self.cur_health += get_max(2);
			if self.cur_health > self.max_health {
				self.cur_health = self.max_health;
			}
			return true;
		}
		false
	}
	pub fn get_image(&self) -> (String,String){
		(

			self.image.clone(),
			self.char_type.to_string()
		)
	}
	pub fn make_action(&self, others:&[CharWithId]) -> Action {
		let others : Vec<&CharWithId> = others.iter().filter(|v| *self != v.character).collect();
		let random_number = get_max(others.len() as i32 ) as usize;
		let on = others[random_number];
		Action {
			on : on.id,
			damage : get_max( self.strength + 2) + 1,
			accuracy : self.accuracy,
			strength : self.strength,
			spectacle_delta : get_between(-1, 1)
		}
	}
}