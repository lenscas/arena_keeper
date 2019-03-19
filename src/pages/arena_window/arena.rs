use crate::agents::router;
use crate::generated::routes;
use crate::agents::fight_agent;
use crate::components::arena::arena_container;
use yew::prelude::*;

pub struct Arena {
	fights : Vec<fight_agent::FightId>,
	arena_worker: Box<Bridge<fight_agent::Worker>>,
	is_open :bool,
	route_worker: Box<Bridge<router::Worker>>,
}

pub enum Msg {
	Toggle,
	Close,
	Router(router::Request),
	UpdateFights(fight_agent::Response),
}
#[derive(PartialEq, Clone)]
pub struct Props {}

impl Default for Props {
	fn default() -> Self {
		Props {
		}
	}
}

impl Component for Arena {
	type Message = Msg;
	type Properties = Props;

	fn create(_props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
		let route_callback = link.send_back(Msg::Router);
		let route_worker = router::Worker::bridge(route_callback);
		let mut comp = Arena {
			fights : Vec::new(),
			arena_worker : fight_agent::Worker::bridge(link.send_back(Msg::UpdateFights)),
			route_worker,
			is_open:true
		};
		comp.arena_worker.send(fight_agent::Request::GetAllFights);
		comp
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {

		match msg {
			Msg::Toggle => {
				self.is_open = !self.is_open;
				true
			},
			Msg::UpdateFights(res) => {
				match res {
					fight_agent::Response::UpdateFightList(list) => {
						self.fights = list;
						true
					},
					_ => {
						unreachable!();
					}
				}
			},
			Msg::Close => {
				self.route_worker.send(router::Request::DeactivateWindow(routes::Windows::Arena));
				false
			},
			Msg::Router(_) => false
		}
	}
	fn change(&mut self, _props: Props) -> ShouldRender{
		true
	}
}

impl Renderable<Arena> for Arena {
	fn view(&self) -> Html<Self> {
		html! {
			<div class="window", style="width:75vw",>
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
								{"arena"}
							</div>
						</div>
					</div>
					<div class="card-body",>
						<arena_container::Arena: />
					</div>
				</div>
			</div>
		}
	}
}
