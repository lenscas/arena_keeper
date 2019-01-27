use crate::agents::character_agent::CharacterId;
use crate::components::character::health_bar::HealthBarProps;
use crate::components::character::health_bar::health_bar;
use yew::prelude::*;

use crate::classes::character::Character;

use crate::agents::character_agent::Worker;
use crate::agents::character_agent::Response;
use crate::agents::character_agent::Request;

use crate::agents::money_agent;

pub struct CharacterListItem {
	character_id : CharacterId,
	character: Option<Character>,
	worker: Box<Bridge<Worker>>,
	money_worker: Box<Bridge<money_agent::Worker>>
}
pub enum Msg {
	Response(Response),
	BuyChar,
	Money
}
#[derive(PartialEq, Clone)]
pub struct Props {
	pub character_id: CharacterId
}
impl Default for Props {
	fn default() -> Self {
		Props {
			character_id: CharacterId {0:0}
		}
	}
}
impl Component for CharacterListItem {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
		let callback = link.send_back(|res| Msg::Response(res));
		let worker = Worker::bridge(callback);

		let mon_callback = link.send_back(|_| Msg::Money);
		let mon_worker = money_agent::Worker::bridge(mon_callback);
		let mut char_item = CharacterListItem {
			character : None,
			worker:worker,
			character_id : props.character_id,
			money_worker: mon_worker
		};
		char_item.worker.send(Request::GetAvailableChar(char_item.character_id));
		char_item
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::BuyChar => {
				self.money_worker.send(money_agent::Request::BuyCharacter(self.character_id));
			},

			Msg::Response(res) => {
				match res {
					Response::AnswerSingleChar(character,_) => {

						self.character = Some(character);
					},
					_default => {
						unreachable!();
					}
				}
			},
			Msg::Money => {

			}
		}
		true
	}
	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		let old_id = self.character_id;
		self.character_id = props.character_id;
		self.character = None;
		self.worker.send(Request::SwitchSubscribedAvailableCharacter(old_id,props.character_id));
		true
	}
}
impl Renderable<CharacterListItem> for CharacterListItem {
	fn view(&self) -> Html<Self> {
		if let Some(character) = &self.character {
			let image = character.get_image();
			let name = character.name.clone();
			let desc = character.description.clone();
			let max_health = character.max_health;
			let cur_health = character.cur_health;
			return html! {
				<li class="list-group-item", onclick=|_|Msg::BuyChar,>
					<div class="row",>
						<div class="col-md-3",>
							//<h1>{self.CharacterId}</h1>
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
								<p>{desc}</p>
							</div>
						</div>
					</div>

				</li>
			}
		}
		return html!{<div></div>}
	}
}