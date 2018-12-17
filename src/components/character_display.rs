
use yew::prelude::*;

use crate::components::character;


pub struct CharacterDisplay {
	character : character::Character
}

pub enum Msg {}
#[derive(PartialEq, Clone)]
pub struct Props {
	pub is_character: character::Character
}

impl Default for Props {
	fn default() -> Self {
		Props {
			is_character: character::Character::create_character(character::CharacterTypes::Human)
		}
	}
}

impl<CTX: 'static> Component<CTX> for CharacterDisplay {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
		CharacterDisplay {
			character : props.is_character
		}
	}
	fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
		match msg {
		}
	}
	fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
		self.character =props.is_character;
		true
	}
}

impl<CTX: 'static> Renderable<CTX, CharacterDisplay> for CharacterDisplay {
	fn view(&self) -> Html<CTX, Self> {
		let amount =  self.character.cur_health * 100 / self.character.max_health;// * 100;
		let width = "width: ".to_owned() + &(amount.to_string()) + "%";
		let color : &str;
		if amount > 50 {
			color = &"bg-success";
		} else if amount > 20 {
			color = &"bg-warning";
		} else {
			color = &"bg-danger";
		}
		html! {
			<li class="list-group-item",>
				<div class="row",>
					<div class="col-md-3",>
						<img class="img-fluid", src={self.character.get_image()},/>
					</div>
					<div class="col",>
						<h5>{self.character.name.clone()}</h5>
						<div class="row",>
							<div class="col-md-9",>
								<div class="progress",>
									<div class=("progress-bar",color), role="progressbar", style={width},/>
								</div>
							</div>
							<div class="col",>
								{"HP"}
							</div>
						</div>
					</div>
				</div>
			</li>
		}
	}
}