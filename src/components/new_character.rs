use crate::components::character::Character;
use crate::components::character::CharacterTypes;
use yew::prelude::*;

pub struct NewCharacter {
	money_left: i64,
	on_buy : Option<Callback<(Character)>>,
	is_human : bool,
}

pub enum Msg {
	BuyChar,
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
			is_human: false,
		}
	}
	fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
		match msg {
			Msg::BuyChar => {
				self.money_left = self.money_left + 1;
				if let Some(ref mut callback) = self.on_buy {
					if self.is_human {
						callback.emit(Character::create_character(CharacterTypes::Human));
					} else {
						callback.emit(Character::create_character(CharacterTypes::Merfolk));
					}
					self.is_human = !self.is_human;
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
						onclick=|_|Msg::BuyChar,
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
							<h5 class="modal-title",>{"Modal title"}</h5>
							<button type="button", class="close", data-dismiss="modal", aria-label="Close",>
								<span aria-hidden="true",>{"&times;"}</span>
							</button>
						</div>
						<div class="modal-body",>
							<p>{"Modal body text goes here."}</p>
						</div>
						<div class="modal-footer",>
							<button type="button", class=("btn","btn-primary"),>{"Save changes"}</button>
							<button type="button", class=("btn","btn-secondary"), data-dismiss="modal",>{"Close"}</button>
						</div>
					</div>
				</div>
			</div>
		}
	
	}
}
