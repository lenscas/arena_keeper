use crate::components::health_bar::HealthBarProps;
use crate::components::health_bar::health_bar;
use crate::components::character::Character;
use crate::components::character::CharacterTypes;
use crate::components::ticker::Worker;
use crate::components::ticker::Request;
use yew::prelude::*;

pub type CharWithId = (Character,i64);
pub struct NewCharacter {
	money_left: i64,
	on_buy : Option<Callback<(Character)>>,
	char_list : Vec<(CharWithId)>,
	worker: Box<Bridge<Worker>>,
	last_id : i64
}

pub enum Msg {
	BuyChar(i64),
	DataReceived,
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

impl Component for NewCharacter {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
		let callback = link.send_back(|_| Msg::DataReceived);
		let worker = Worker::bridge(callback);
		NewCharacter {
			on_buy: props.on_buy,
			money_left : 100,
			char_list : vec![
				(Character::create_character(CharacterTypes::Human),1),
				(Character::create_character(CharacterTypes::Merfolk),2),
				(Character::create_character(CharacterTypes::Human),3)
			],
			worker : worker,
			last_id : 3

		}
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {

		match msg {
			Msg::BuyChar(character_id) => {
				self.worker.send(Request::Question(String::from("Hello?")));
				if self.money_left < 100 {
					return false;
				}
				let maybe_char = self.char_list.iter().find(|x| x.1 == character_id);
				match maybe_char {
					Some(character) => {
						self.money_left = self.money_left - 100;
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
			Msg::DataReceived => {
				if self.money_left < 100 {
					self.money_left = self.money_left + 50;
				}
				if self.char_list.len() < 3 {
					self.last_id = self.last_id + 1;
					self.char_list.push( (Character::create_character(CharacterTypes::Human),self.last_id))
				}
				true
			}
		}
	}
	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.on_buy = props.on_buy;
		false
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
										|character| self.render_character_selection(character.to_owned())
									)
							}
						</div>
					</div>
				</div>
			</div>
		}
	}

	fn render_character_selection(&self, character : CharWithId) -> Html<Self> {
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
