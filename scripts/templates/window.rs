use crate::agents::router;
use crate::generated::routes;
//use crate::classes::character::Character;

use yew::prelude::*;

pub struct {{WINDOW_NAME_CAPS}} {
	is_open :bool,
	route_worker: Box<Bridge<router::Worker>>,
}

pub enum Msg {
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

impl Component for {{WINDOW_NAME_CAPS}} {
	type Message = Msg;
	type Properties = Props;

	fn create(_props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
		let route_callback = link.send_back(|res| Msg::Router(res));
		let route_worker = router::Worker::bridge(route_callback);
		{{WINDOW_NAME_CAPS}} {
			route_worker,
			is_open:true
		}
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {

		match msg {
			Msg::Toggle => {
				self.is_open = !self.is_open;
				true
			},
			Msg::Close => {
				self.route_worker.send(router::Request::DeactivateWindow(routes::Windows::{{WINDOW_NAME_CAPS}}));
				false
			},
			Msg::Router(_) => false
		}
	}
	fn change(&mut self, _props: Props) -> ShouldRender{
		true
	}
}

impl Renderable<{{WINDOW_NAME_CAPS}}> for {{WINDOW_NAME_CAPS}} {
	fn view(&self) -> Html<Self> {
		html! {
			<div class="window",>
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
								{"{{WINDOW_NAME}}"}
							</div>
						</div>
					</div>
					<div class="card-body",>
					</div>
				</div>
			</div>
		}
	}
}
