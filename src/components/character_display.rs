
use yew::prelude::*;

use crate::components::character;
use crate::components::health_bar::health_bar;
use crate::components::health_bar::HealthBarProps;

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
		let image = self.character.get_image();
		html! {
			<li class="list-group-item",>
				<div class="row",>
					<div class="col-md-3",>
						<img class="img-fluid",alt={image.1}, src={image.0},/>
					</div>
					<div class="col",>
						<h5>{self.character.name.clone()}</h5>
						<div class="row",>
							<div class="col-md-9",>
								{
									health_bar(
										HealthBarProps {
											max: self.character.max_health,
											current:self.character.cur_health,
											break_yellow:50,
											break_red:20,
										}
									)
								}
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