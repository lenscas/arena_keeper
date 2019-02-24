use crate::agents::character_agent::CharacterId;
use crate::agents::money_agent;

use crate::pages::shop_window::new_character_item::CharacterListItem;
use crate::agents::router;
use crate::generated::routes::Windows;
//use crate::classes::character::Character;

use yew::prelude::*;

pub struct Shop {
	money_left: i64,
	char_list : Vec<(CharacterId)>,
	money_worker: Box<Bridge<money_agent::Worker>>,
	is_open :bool,
	route_worker: Box<Bridge<router::Worker>>,
}

pub enum Msg {
	NewMoney(money_agent::Response),
	Toggle,
	Close,
	Router(router::Request)
}
#[derive(PartialEq, Clone)]
pub struct Props {}

impl Default for Props {
	fn default() -> Self {
		Props {
		}
	}
}

impl Component for Shop {
	type Message = Msg;
	type Properties = Props;

	fn create(_props: Self::Properties, mut link: ComponentLink<Self>) -> Self {

		let money_callback = link.send_back(Msg::NewMoney);
		let money_worker = money_agent::Worker::bridge(money_callback);

		let route_callback = link.send_back(Msg::Router);
		let route_worker = router::Worker::bridge(route_callback);
		let mut new_char = Shop {
			money_left : 0,
			char_list : vec![],
			money_worker: money_worker,
			route_worker,
			is_open:true
		};
		new_char.money_worker.send(money_agent::Request::GetList);
		new_char
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {

		match msg {
			/*
			Msg::GetList(action) => {
				match action {

					_default => {
						unreachable!();
					}
				}
				true
			},
			*/
			Msg::Toggle => {
				self.is_open = !self.is_open;
				true
			},
			Msg::Close => {
				self.route_worker.send(router::Request::DeactivateWindow(Windows::Shop));
				false
			},
			Msg::NewMoney(res) => {
				match res {
					money_agent::Response::AnswerIdList(list) => {
						self.char_list = list;
						true
					},
					money_agent::Response::NewAmount(money) => {
						self.money_left = money;
						true
					}
					_ => false
				}
			},
			Msg::Router(_) => false
		}
	}
	fn change(&mut self, _props: Props) -> ShouldRender{
		true
	}
}

impl Renderable<Shop> for Shop {
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
								<h4>
									{"Shop"}
								</h4>
							</div>
							<div class="col",>
								<span>
									{self.money_left}
								</span>
							</div>
						</div>
					</div>
					{self.render_list()}
				</div>
			</div>
		}
	}
}
impl Shop {
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