
use crate::components::shared::basic_char_render::basic_char_render;
use crate::classes::character::Character;
use crate::classes::fight::Fight;
use crate::agents::fight_agent;
use crate::agents::character_agent;
use yew::prelude::*;

pub struct FightItem {
	fight : Option<Fight>,
	chars : Option<( (character_agent::CharacterId,Character),(character_agent::CharacterId,Character))>,
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
		let fight_callback = link.send_back(|res| Msg::UpdateFights(res));
		let fight_worker = fight_agent::Worker::bridge(fight_callback);

		let char_callback = link.send_back(|res| Msg::UpdateCharacter(res));
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
						self.char_worker.send(character_agent::Request::GetDoubleCharacter(fight.get_fighters_ids()));
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
					character_agent::Response::AnswerSingleChar(chara,char_id) => {
						if let Some(mut current_chars) = self.chars.clone() {
							if (current_chars.0).0 == char_id {
								(current_chars.0).1 = chara;
							} else {
								(current_chars.1).1 = chara;
							};
							self.chars = Some(current_chars);
						};
						true
					},
					character_agent::Response::AnswerDoubleChar(char1,char2) => {
						self.chars = Some((char1,char2));
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
		if let Some(fight) = &self.fight {
			if let Some(characters) = &self.chars {
				html! {
					<li class="list-group-item",>
						<div class="row",>
							<div class="col-md-4",>
								{basic_char_render( (characters.0).1.clone())}
							</div>
							<div class=("col-md-4","text-center"),>
								<div class=("row","justify-content-center"),>
									<div class="col",>
										<h2>{"VS"}</h2>
									</div>
								</div>
								<div class="row justify-content-center",>
									<div class="col",>
										{fight.get_started_text()}
									</div>
								</div>
							</div>
							<div class="col-md-4",>
								{basic_char_render( (characters.1).1.clone())}
							</div>

						</div>
					</li>
				}
			} else {
				html! {<></>}
			}

		} else {
			html! {<></>}
		}

	}
}
