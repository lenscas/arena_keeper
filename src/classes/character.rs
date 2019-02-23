use crate::generated::create_description::generate_description;
use crate::generated::create_name::generate_name;
use crate::generated::species_types::SpeciesTypes;
use crate::generated::create_type::generate_type;
use crate::generated::genereate_image::generate_image;

#[derive(Eq,Hash,PartialEq,Clone,Serialize, Deserialize,Debug)]
pub struct Character {
	pub char_type : SpeciesTypes,
	pub name : String,
	pub max_health: i64,
	pub cur_health : i64,
	pub image:String,
	pub description : String,
	pub is_fighting : bool
}
impl Character {
	pub fn create_character() -> Character {
		let species = generate_type();
		let name = generate_name(species);
		let image = generate_image(species);
		let description = generate_description(species);
		Character {
			char_type : species,
			name : name,
			max_health : 100,
			cur_health :100,
			image,
			description,
			is_fighting : false
		}
	}
	pub fn update(&mut self) -> bool {
		if self.cur_health < 100 && self.cur_health > 0 && !self.is_fighting {
			self.cur_health += 5;
			if self.cur_health > 100 {
				self.cur_health = 100;
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
}