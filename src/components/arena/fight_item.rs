
use crate::funcs::array_helper::ensure_all_some_chars;
use crate::components::shared::basic_char_render::basic_char_render;
use crate::classes::fight::Fight;
use crate::classes::character::Character;
use crate::agents::fight_agent;
use crate::agents::character_agent;

use yew::prelude::*;

struct RenderFight {
	fight : Fight,
	char_1 : Character,
	char_2 : Character
}

pub struct FightItem {
	fight : Option<Fight>,
	chars : Option<Vec<character_agent::MaybeCharWithId>>,
	fight_id : fight_agent::FightId,
	fight_worker: Box<Bridge<fight_agent::Worker>>,
	char_worker : Box<Bridge<character_agent::Worker>>
}


pub enum Msg {
	UpdateFights(fight_agent::Response),
	UpdateCharacter(character_agent::Response)
}
#[derive(PartialEq, Clone)]
pub struct Props {
	pub fight_id : fight_agent::FightId
}

impl Default for Props {
	fn default() -> Self {
		Props {
			fight_id : fight_agent::FightId {0:0}
		}
	}
}
impl Component for FightItem
{
	type Message = Msg;
	type Properties = Props;
	fn create(props: Self::Properties, mut link:  ComponentLink<Self>) -> Self {
		let fight_callback = link.send_back(Msg::UpdateFights);
		let fight_worker = fight_agent::Worker::bridge(fight_callback);

		let char_callback = link.send_back(Msg::UpdateCharacter);
		let char_worker = character_agent::Worker::bridge(char_callback);
		let mut fight_item = FightItem {
			chars : None,
			fight : None,
			fight_id : props.fight_id,
			fight_worker,
			char_worker
		};
		fight_item.fight_worker.send(fight_agent::Request::GetFight(props.fight_id));
		fight_item
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::UpdateFights(res) => {
				match res {
					fight_agent::Response::UpdateFight(fight) => {
						self.char_worker.send(character_agent::Request::GetMultipleCharacters(fight.get_fighters_ids()));
						self.fight = Some(fight);
						true
					},
					_ => {
						unreachable!();
					}
				}
			},
			Msg::UpdateCharacter(res) => {
				match res {
					character_agent::Response::AnswerSingleChar(character) => {
						if let Some(current_chars) = self.chars.clone() {
							self.chars = Some(
								current_chars
									.iter()
									.cloned()
									.map(
										|v|
										if v.id == character.id {
											character.clone()
										} else {
											v
										}
									).collect()
							);
						};
						true
					},
					character_agent::Response::AnswerMultipleChars(new_characters) => {
						self.chars = Some(new_characters);
						true
					},
					_ => {
						unreachable!();
					}
				}
			}
		}
	}
	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.fight = None;
		self.fight_worker.send(fight_agent::Request::GetFight(props.fight_id));
		self.fight_id = props.fight_id;
		true
	}
}
impl Renderable<FightItem> for FightItem
{
	fn view(&self) -> Html<Self> {
		let m_fight = self.can_render();
		if let Some(fight) = m_fight {
			return html! {
				<li class="list-group-item",>
					<div class="row",>
						<div class="col-md-4",>
							{basic_char_render( fight.char_1.clone())}
						</div>
						<div class=("col-md-4","text-center"),>
							<div class=("row","justify-content-center"),>
								<div class="col",>
									<h2>{"VS"}</h2>
								</div>
							</div>
							<div class="row justify-content-center",>
								<div class="col",>
									{fight.fight.get_started_text()}
								</div>
							</div>
						</div>
						<div class="col-md-4",>
							{basic_char_render( fight.char_2.clone())}
						</div>
					</div>
				</li>
			}
		}
		html!{<h1>{"Test"}</h1>}
	}

}
impl FightItem {
	fn can_render(&self) -> Option<RenderFight> {
		if let Some(fight) = &self.fight {
			if let Some(maybe_chars) = &self.chars {
				let m_chars = ensure_all_some_chars(maybe_chars);
				if let Some(chars) = m_chars {
					if let Some(char_1) = chars.get(0) {
						if let Some(char_2) = chars.get(1) {
							return Some(
								RenderFight {
									fight : fight.to_owned(),
									char_1 : char_1.character.to_owned(),
									char_2 : char_2.character.to_owned()
								}
							)
						}
					}
				}
			}
		}
		None
	}
}

