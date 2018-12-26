#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate yew;

use yew::prelude::*;

mod components;

use crate::components::character_list::CharacterList;

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
					</div>
				</div>
			</>
		}
	}
}