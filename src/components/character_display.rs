
use yew::prelude::*;

use crate::classes::character;
use crate::components::health_bar::health_bar;
use crate::components::health_bar::HealthBarProps;

use crate::agents::character_agent::Worker;
use crate::agents::character_agent::Request;
use crate::agents::character_agent::Response;

pub struct CharacterDisplay {
	character : Option<character::Character>,
	worker: Box<Bridge<Worker>>,
	character_id : i64
}

pub enum Msg {
	Update(Response)
}
#[derive(PartialEq, Clone)]
pub struct Props {
	pub character_id: i64
}

impl Default for Props {
	fn default() -> Self {
		Props {
			character_id: 0
		}
	}
}

impl Component for CharacterDisplay {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
		let callback = link.send_back(|res| Msg::Update(res));
		let worker = Worker::bridge(callback);
		let mut char_display = CharacterDisplay {
			character : None,
			worker:worker,
			character_id: props.character_id
		};

		char_display.worker.send(Request::GetCharacter(char_display.character_id));
		char_display
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Update(res) => {
				js!{console.log("in answer?")};
				match res {
					Response::AnswerSingleChar(chara) => {

						self.character = Some(chara);
					},
					_default => {
						unreachable!();
					}
				}
			}
		}
		true
	}
	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		let old_id = self.character_id;
		//self.character_id = props.character_id;
		//self.character = None;
		//self.worker.send(Request::GetAvailableChar(props.character_id));
		self.worker.send(Request::SwitchSubscribedCharacter(old_id,props.character_id));
		true
	}
}

impl Renderable<CharacterDisplay> for CharacterDisplay {
	fn view(&self) -> Html<Self> {
		if let Some(character) = &self.character {
			let image = character.get_image();
			return html! {
				<li class="list-group-item",>
					<div class="row",>
						<div class="col-md-3",>
							<img class="img-fluid",alt={image.1}, src={image.0},/>
						</div>
						<div class="col",>
							<h5>{character.name.clone()}</h5>
							<div class="row",>
								<div class="col-md-9",>
									{
										health_bar(
											HealthBarProps {
												max: character.max_health,
												current:character.cur_health,
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
		return html! {
			<div>{self.character_id}</div>
		}

	}
}