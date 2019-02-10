use yew::prelude::*;
use crate::generated::routes::Windows;
use std::collections::HashSet;

use crate::agents::router;

{{LOAD_CRATES}}

pub struct ActiveWindows {
	_router: Box<Bridge<router::Worker>>,
	active_windows: HashSet<Windows>
}

pub enum Msg {
	HandleWindowState( router::Request)
}

impl Component for ActiveWindows{
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, mut link: ComponentLink<Self> ) -> Self {
		let callback = link.send_back(|route| Msg::HandleWindowState(route));
		let router = router::Worker::bridge(callback);
		ActiveWindows {
			active_windows: HashSet::new(),
			_router :router,
		}
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::HandleWindowState(window) => {
				match window {
					router::Request::ActivateWindow(w) => self.active_windows.insert(w),
					router::Request::DeactivateWindow(w) => self.active_windows.remove(&w)
				}
			}
		}
	}
	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		true
	}
}
impl Renderable<ActiveWindows> for ActiveWindows{
	fn view(&self) -> Html<Self> {
		html!{
			{
				for(self.active_windows.iter()).map(
					|w|
					match w {
						{{RENDER_WINDOWS}}
					}
				)
			}
		}
	}
}
