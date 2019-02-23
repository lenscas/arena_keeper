
use crate::components::shared::basic_char_render::basic_char_render;
use yew::prelude::*;

use crate::classes::character;

use crate::agents::character_agent::Worker;
use crate::agents::character_agent::Request;
use crate::agents::character_agent::Response;
use crate::agents::character_agent::CharacterId;

use crate::agents::fight_agent;

pub struct CharacterDisplay {
	character : Option<character::Character>,
	worker: Box<Bridge<Worker>>,
	arena: Box<Bridge<fight_agent::Worker>>,
	character_id : CharacterId
}

pub enum Msg {
	Update(Response),
	UpdateArena(fight_agent::Response),
	Click
}
#[derive(PartialEq, Clone)]
pub struct Props {
	pub character_id: CharacterId
}

impl Default for Props {
	fn default() -> Self {
		Props {
			character_id: CharacterId { 0 : 0}
		}
	}
}

impl Component for CharacterDisplay {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
		let callback = link.send_back(Msg::Update);
		let worker = Worker::bridge(callback);

		let fight_callback = link.send_back(Msg::UpdateArena);
		let fight_worker = fight_agent::Worker::bridge(fight_callback);
		let mut char_display = CharacterDisplay {
			character : None,
			worker:worker,
			arena:fight_worker,
			character_id: props.character_id
		};

		char_display.worker.send(Request::GetCharacter(char_display.character_id));
		char_display
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Update(res) => {
				match res {
					Response::AnswerSingleChar(chara) => {

						self.character = chara.character;
					},
					_default => {
						unreachable!();
					}
				}
			},
			Msg::UpdateArena(_res)=>{

			}
			Msg::Click => {
				self.arena.send(fight_agent::Request::AddAsFighter(self.character_id));
			}
		}
		true
	}
	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		let old_id = self.character_id;
		self.worker.send(Request::SwitchSubscribedCharacter(old_id,props.character_id));
		true
	}
}

impl Renderable<CharacterDisplay> for CharacterDisplay {
	fn view(&self) -> Html<Self> {
		if let Some(character) = &self.character {
			return html! {
				<li class="list-group-item", onclick=|_|Msg::Click ,>
					{
						basic_char_render(character.clone())
					}
				</li>
			}
		}
		html!{<></>}
	}
}