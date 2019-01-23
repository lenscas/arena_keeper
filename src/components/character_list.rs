use yew::prelude::*;

use crate::components::new_character::NewCharacter;
use crate::components::character_display::CharacterDisplay;
use crate::agents::character_agent::Worker;
use crate::agents::character_agent::Request;
use crate::agents::character_agent::Response;

pub struct CharacterList {
	characters : Vec<i64>,
	worker: Box<Bridge<Worker>>,
}

pub enum Msg {
	UpdateChars(Response)
}
impl Component for CharacterList
{
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
		let callback = link.send_back(|req| Msg::UpdateChars(req));
		let worker = Worker::bridge(callback);
		let mut char_list = CharacterList {
			characters : vec!(),
			worker:worker
		};
		char_list.worker.send(Request::GetIdList);
		char_list
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::UpdateChars(req) => {
				match req {
					Response::AnswerIdList(id_list) => {
						self.characters = id_list;
					},
					_default => {
						unreachable!();
					}
				}
			}
		}
		true
	}
}
impl Renderable<CharacterList> for CharacterList
{
	fn view(&self) -> Html<Self> {
		js!{console.log("RERENDER!")}
		html! {
			<div class=("card","h-100"),>
				<div class=("card-header","h-10"),>
					<NewCharacter: />
				</div>
				<ul class=("list-group","list-item-flush","h-90", "scrollBar"),>
					{
						for(self.characters).iter().map(
							|character_id| return html! {
								<CharacterDisplay: character_id=character_id,/>
							}
						)
					}
				</ul>
			</div>
		}
	}
}

impl CharacterList {}
