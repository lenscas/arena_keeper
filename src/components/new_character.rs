use crate::components::health_bar::HealthBarProps;
use crate::components::health_bar::health_bar;
use crate::components::character::Character;
use crate::components::character::CharacterTypes;

use yew::prelude::*;

pub type CharWithId = (Character,i64);
pub struct NewCharacter {
	money_left: i64,
	on_buy : Option<Callback<(Character)>>,
	char_list : Vec<(CharWithId)>
}

pub enum Msg {
	BuyChar(i64),
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
			char_list : vec![
				(Character::create_character(CharacterTypes::Human),1),
				(Character::create_character(CharacterTypes::Merfolk),2),
				(Character::create_character(CharacterTypes::Human),3)
			]
		}
	}
	fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
		match msg {
			Msg::BuyChar(character_id) => {
				let maybe_char = self.char_list.iter().find(|x| x.1 == character_id);
				match maybe_char {
					Some(character) => {
						self.money_left = self.money_left + 1;
						if let Some(ref mut callback) = self.on_buy {
							callback.emit(character.clone().0);
							let new_list = self.char_list.iter()
								.cloned()
								.filter(
									|x| x.1 != character_id
								).collect::<Vec<CharWithId>>();
							self.char_list = vec![];
							self.char_list.extend(new_list);
						}

					},
					None => {
						
					}
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
							{
								for(self.char_list.iter())
									.map(
										|character| self.render_character_selection(character.to_owned())
									)
							}
						</div>
					</div>
				</div>
			</div>
		}
	}

	fn render_character_selection<CTX: 'static>(&self, character : CharWithId) -> Html<CTX,Self> {
		let image = character.0.get_image();
		let name = character.0.name.clone();
		let max_health = character.0.max_health;
		let cur_health = character.0.cur_health;
		html! {
			<li class="list-group-item", onclick=|_|Msg::BuyChar(character.1),>
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
