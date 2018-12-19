use crate::components::health_bar::HealthBarProps;
use crate::components::health_bar::health_bar;
use crate::components::character::Character;
use crate::components::character::CharacterTypes;

use yew::prelude::*;

pub struct NewCharacter {
	money_left: i64,
	on_buy : Option<Callback<(Character)>>,
}

pub enum Msg {
	BuyChar(Character),
}
#[derive(PartialEq, Clone)]
pub struct Props {
	pub on_buy : Option<Callback<(Character)>>
}

impl Default for Props {
	fn default() -> Self {
		Props {
			on_buy: None
		}
	}
}

impl<CTX: 'static> Component<CTX> for NewCharacter {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
		NewCharacter {
			on_buy: props.on_buy,
			money_left : 0,
		}
	}
	fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
		match msg {
			Msg::BuyChar(character) => {
				self.money_left = self.money_left + 1;
				if let Some(ref mut callback) = self.on_buy {
					callback.emit(character.clone());
				}
				true
			},
		}
	}
	fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
		self.on_buy = props.on_buy;
		false
	}
}

impl<CTX: 'static> Renderable<CTX, NewCharacter> for NewCharacter {
	fn view(&self) -> Html<CTX, Self> {
		html! {
			<div class="row",>
				<div class="col",>
					<span>
						{self.money_left}
					</span>
				</div>
				<div class="col",>
					<button
						class="btn",
						class="btn-success",
						data-toggle="modal",
						data-target="#charSelectModal",
					>
						{"Add new"}
					</button>
				</div>
				{self.render_modal()}
			</div>
		}
	}
}
impl NewCharacter {
	fn render_modal<CTX: 'static>(&self) -> Html<CTX,Self> {
		html! {
			<div class="modal", tabindex="-1", id="charSelectModal", role="dialog",>
				<div class="modal-dialog", role="document",>
					<div class="modal-content",>
						<div class="modal-header",>
							<h5 class="modal-title",>{"Buy character"}</h5>
							<button type="button", class="close", data-dismiss="modal", aria-label="Close",>
								<span aria-hidden="true",>{"X"}</span>
							</button>
						</div>
						<div class="modal-body",>
							{self.render_character_selection(CharacterTypes::Human)}
							{self.render_character_selection(CharacterTypes::Merfolk)}
							{self.render_character_selection(CharacterTypes::Human)}
						</div>
					</div>
				</div>
			</div>
		}
	}

	fn render_character_selection<CTX: 'static>(&self, char_type : CharacterTypes) -> Html<CTX,Self> {
		let character = Character::create_character(char_type);
		let image = character.get_image();
		let name = character.name.clone();
		let max_health = character.max_health;
		let cur_health = character.cur_health;
		html! {
			<li class="list-group-item", onclick=|_|Msg::BuyChar(character.to_owned()),>
				<div class="row",>
					<div class="col-md-3",>
						<img class="img-fluid",alt={image.1}, src={image.0},/>
					</div>
					<div class="col",>
						<h5>{name.clone()}</h5>
						<div class="row",>
							<div class="col-md-9",>
								{
									health_bar(
										HealthBarProps {
											max: max_health,
											current: cur_health,
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
						<div class="row",>
							<p>{"This is a nice description of this race"}</p>
						</div>
					</div>
				</div>

			</li>
		}
	}
}
