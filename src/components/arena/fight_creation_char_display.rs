use crate::components::shared::basic_char_render::basic_char_render;
use crate::classes::character::Character;
use crate::agents::fight_agent;
use crate::agents::character_agent;

use yew::prelude::*;


pub struct SideCharDisplay {
	fight_worker :  Box<Bridge<fight_agent::Worker>>,
	character_worker : Box<Bridge<character_agent::Worker>>,
	character : Option<Character>,
	char_id : Option<character_agent::CharacterId>
}


pub enum Msg {
	UpdateFights(fight_agent::Response),
	UpdateCharacter(character_agent::Response)
}
#[derive(PartialEq, Clone)]
pub struct Props {
	pub is_left : bool,
}

impl<'a> Default for Props {
	fn default() -> Self {
		Props {
			is_left : true,
		}
	}
}
impl Component for SideCharDisplay
{
	type Message = Msg;
	type Properties = Props;
	fn create(props: Self::Properties, mut link:  ComponentLink<Self>) -> Self {
		let fight_callback = link.send_back(|res| Msg::UpdateFights(res));
		let fight_worker = fight_agent::Worker::bridge(fight_callback);

		let character_callback = link.send_back(|res| Msg::UpdateCharacter(res));
		let character_worker = character_agent::Worker::bridge(character_callback);


		let mut ar = SideCharDisplay {
			fight_worker,
			character_worker,
			character : None,
			char_id : None
		};
		let num = if props.is_left {
			0
		} else {
			1
		};
		ar.fight_worker.send(fight_agent::Request::GetReadyFighter(num));
		ar
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::UpdateFights(res) => {
				match res {
					fight_agent::Response::UpdateFighter(new_char) => {
						if let Some(chara) = new_char {
							if let Some(old_char) = self.char_id {
								self.character_worker.send(character_agent::Request::SwitchSubscribedCharacter(old_char,chara));
							} else {
								self.character_worker.send(character_agent::Request::GetCharacter(chara));
							}
							 //.send();
						} else {
							info!("in none");
							self.character = None;
						}
						self.char_id = new_char;
					},
					_ => {
						unreachable!();
					}
				}
				true
			},
			Msg::UpdateCharacter(res) => {
				match res {
					character_agent::Response::AnswerSingleChar(new_char,_) => {
						self.character = Some(new_char);
						true
					},
					_ => {
						false
					}
				}
			}
		}
	}
	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		true
	}
}
impl Renderable<SideCharDisplay> for SideCharDisplay
{
	fn view(&self) -> Html<Self> {
		if let Some(character) = self.character.clone() {
			basic_char_render(character)
		} else {
			html!{<></>}
		}

	}
}
