use crate::agents::character_agent;
use crate::agents::money_agent;

use crate::components::new_character_list_item::CharacterListItem;
use crate::classes::character::Character;

use yew::prelude::*;

pub type CharWithId = (Character,i64);
pub struct NewCharacter {
	money_left: i64,
	char_list : Vec<(i64)>,
	_money_worker: Box<Bridge<money_agent::Worker>>,
	char_worker: Box<Bridge<character_agent::Worker>>,
}

pub enum Msg {
	GetList(character_agent::Response),
	NewMoney(money_agent::Response),
}
#[derive(PartialEq, Clone)]
pub struct Props {}

impl Default for Props {
	fn default() -> Self {
		Props {
		}
	}
}

impl Component for NewCharacter {
	type Message = Msg;
	type Properties = Props;

	fn create(_props: Self::Properties, mut link: ComponentLink<Self>) -> Self {

		let money_callback = link.send_back(|res| Msg::NewMoney(res));
		let money_worker = money_agent::Worker::bridge(money_callback);
		let character_agent_callback = link.send_back(|ids| Msg::GetList(ids));
		let character_worker = character_agent::Worker::bridge(character_agent_callback);
		let mut new_char = NewCharacter {
			money_left : 0,
			char_list : vec![],
			_money_worker: money_worker,
			char_worker : character_worker

		};
		new_char.char_worker.send(character_agent::Request::GetAvailableList);
		new_char
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {

		match msg {
			Msg::GetList(action) => {
				match action {
					character_agent::Response::AnswerIdList(list) => {
						let len = list.len().to_string();
						js!{console.log(@{len})}
						self.char_list = list;
					},
					_default => {
						unreachable!();
					}
				}
				true
			}
			Msg::NewMoney(res) => {
				match res {
					money_agent::Response::NewAmount(money) => {
						self.money_left = money;
						true
					}
				}
			}
		}
	}
	fn change(&mut self, _props: Props) -> ShouldRender{
		true
	}
}

impl Renderable<NewCharacter> for NewCharacter {
	fn view(&self) -> Html<Self> {
		html! {
			<div class="row",>
				<div class="col",>
					<span>
						{self.money_left}
					</span>
				</div>
				<div class="col",>
					<button
						class=("btn","btn-success"),
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
	fn render_modal(&self) -> Html<Self> {
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
										|character| html! {
											<CharacterListItem: character_id={character.to_owned()}, />
										}
									)

							}
						</div>
					</div>
				</div>
			</div>
		}
	}
}