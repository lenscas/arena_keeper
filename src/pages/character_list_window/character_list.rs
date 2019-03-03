use crate::pages::character_list_window::single_character::CharacterListItem;
use crate::agents::character_agent::CharacterId;
use crate::agents::router;
use crate::generated::routes;
use crate::agents::character_agent;
//use crate::classes::character::Character;

use yew::prelude::*;

pub struct CharacterList {
	char_list : Vec<(CharacterId)>,
	character_worker: Box<Bridge<character_agent::Worker>>,
	is_open :bool,
	route_worker: Box<Bridge<router::Worker>>,
}

pub enum Msg {
	Toggle,
	Close,
	Router(router::Request),
	Characters(character_agent::Response)
}
#[derive(PartialEq, Clone)]
pub struct Props {}

impl Default for Props {
	fn default() -> Self {
		Props {
		}
	}
}

impl Component for CharacterList {
	type Message = Msg;
	type Properties = Props;

	fn create(_props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
		let route_callback = link.send_back(Msg::Router);
		let route_worker = router::Worker::bridge(route_callback);
		
		let character_worker = character_agent::Worker::bridge(
			link.send_back(Msg::Characters)
		);
		let mut list = CharacterList {
			character_worker,
			char_list : Vec::new(),
			route_worker,
			is_open:true
		};
		list.character_worker.send(character_agent::Request::GetIdList);
		list
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {

		match msg {
			Msg::Toggle => {
				self.is_open = !self.is_open;
				true
			},
			Msg::Close => {
				self.route_worker.send(router::Request::DeactivateWindow(routes::Windows::CharacterList));
				false
			},
			Msg::Characters(route) => {
				match route {
					character_agent::Response::AnswerIdList(list) => {
						info!("Test?");
						self.char_list = list;
						true
					}
					_ => unreachable!()
				}
			}
			Msg::Router(_) => false
		}
	}
	fn change(&mut self, _props: Props) -> ShouldRender{
		true
	}
}

impl Renderable<CharacterList> for CharacterList {
	fn view(&self) -> Html<Self> {
		html! {
			<div class="shop window",>
				<div class="card",>
					<div class="window-header",>
						<div class="row",>
							<div class="col-md-1 text-left",>
								<p class="bg-danger pointer", onclick=|_|Msg::Close, >{"X"}</p>
							</div>
							<div class="col-md-1 text-left",>
								<p class="bg-warning pointer", onclick=|_|Msg::Toggle, >{"_"}</p>
							</div>
							<div class="col",>
								{"Characters"}
							</div>
						</div>
					</div>
					{self.render_list()}
				</div>
			</div>
		}
	}
}
impl CharacterList {
	fn render_list(&self) -> Html<Self> {
		if ! self.is_open {
			return html! {<></>}
		}
		html! {
			<div class="card-body",>
				<ul class="list-group list-item-flush",>
					{
						for(self.char_list.iter())
							.map(
								|character| html! {
									<CharacterListItem: character_id={character.to_owned()}, />
								}
							)

					}
				</ul>
			</div>
		}
	}
}