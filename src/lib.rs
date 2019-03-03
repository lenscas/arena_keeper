#![recursion_limit="128"]
#![allow(unknown_lints)]
#![warn(clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate yew;
extern crate indexmap;

use crate::components::shared::link::Link;
use crate::generated::active_windows::ActiveWindows;
use crate::agents::router::Request::ActivateWindow;
use crate::generated::routes::Windows;
use crate::pages::index::index;

use yew::prelude::*;

pub mod components;
pub mod agents;
pub mod classes;
pub mod generated;
pub mod funcs;
pub mod pages;

use crate::agents::router;

pub struct Model {
	_router: Box<Bridge<router::Worker>>,
}

pub enum Msg {
	HandleWindowState( router::Request)
}

impl Component for Model{
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, mut link: ComponentLink<Self> ) -> Self {
		let callback = link.send_back(Msg::HandleWindowState);
        let router = router::Worker::bridge(callback);
		Model {
			_router :router,
		}
	}
	fn update(&mut self, _: Self::Message) -> ShouldRender {
		false
	}
	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		true
	}
}
impl Renderable<Model> for Model{
	fn view(&self) -> Html<Self> {
		html! {
			<>
				<nav class=("navbar","navbar-expand-lg", "navbar-dark", "bg-dark"), id="mainNav",>
					<span class="navbar-brand",>{"Arena keeper"}</span>
					<Link:
						action=ActivateWindow(Windows::CharacterList),
						class="btn btn-success",
						text="Shop",
					/>
				</nav>
				{index()}
				<ActiveWindows:/>
			</>
		}
	}
}