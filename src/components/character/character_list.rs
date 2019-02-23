use crate::components::shared::link::Link;
use crate::agents::character_agent::CharacterId;
use yew::prelude::*;

use crate::components::character::character_display::CharacterDisplay;
use crate::agents::character_agent::Worker;
use crate::agents::character_agent::Request;
use crate::agents::character_agent::Response;
use crate::agents::router;
use crate::generated::routes::Windows;

pub struct CharacterList {
	characters : Vec<CharacterId>,
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
		let callback = link.send_back(Msg::UpdateChars);
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
	fn change(&mut self,_: Self::Properties) -> ShouldRender {
		true
	}
}
impl Renderable<CharacterList> for CharacterList
{
	fn view(&self) -> Html<Self> {
		html! {
			<div class=("card","h-100"),>
				<div class=("card-header","h-10"),>
					<Link: action=router::Request::ActivateWindow(Windows::Shop), class="btn btn-success", text="Shop",/>
				</div>
				<ul class=("list-group","list-item-flush","h-90", "scrollBar"),>
					{
						for(self.characters).iter().map(
							|char_id| html! {
								<CharacterDisplay: character_id=char_id,/>
							}
						)
					}
				</ul>
			</div>
		}
	}
}
