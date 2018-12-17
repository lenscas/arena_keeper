use crate::components::character::Character;
use crate::components::character::CharacterTypes;
use yew::prelude::*;

pub struct NewCharacter {
	money_left: i64,
	on_buy : Option<Callback<(Character)>>,
	is_human : bool
}

pub enum Msg {
	BuyChar
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
			is_human: false
		}
	}
	fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
		match msg {
			Msg::BuyChar => {
				self.money_left = self.money_left + 1;
				if let Some(ref mut callback) = self.on_buy {
					println!("did get here?");
					if self.is_human {
						callback.emit(Character::create_character(CharacterTypes::Human));
					} else {
						callback.emit(Character::create_character(CharacterTypes::Merfolk));
					}
					self.is_human = !self.is_human;
				}
				false
			}
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
						onclick=|_| Msg::BuyChar,
					>
						{"Add new"}
					</button>
				</div>
			</div>
		}
	}
}