#![recursion_limit="128"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate yew;

use crate::components::arena::arena_container::Arena;
use yew::prelude::*;

pub mod components;
pub mod agents;
pub mod classes;

use crate::components::character::character_list::CharacterList;

pub struct Model {}

pub enum Msg {}

impl Component for Model{
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, _: ComponentLink<Self> ) -> Self {
		Model {}
	}
	fn update(&mut self, _msg: Self::Message) -> ShouldRender {
		true
	}
}
impl Renderable<Model> for Model{
	fn view(&self) -> Html<Self> {
		html! {
			<>
				<nav class=("navbar","navbar-expand-lg", "navbar-dark", "bg-dark"), id="mainNav",>
					<span class="navbar-brand",>{"Arena keeper"}</span>
				</nav>
				<div class="container-fluid", id="mainPage",>
					<div class=("row","h-100"),>
						<div class=("col-md-3","h-100"),>
							<CharacterList: />
						</div>
						<div class=("col-md-9","h-100"),>
							<Arena: />
						</div>
					</div>
				</div>
			</>
		}
	}
}