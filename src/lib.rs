#[macro_use]
extern crate yew;

use yew::prelude::*;

mod components;

use crate::components::character_list::CharacterList;

pub struct Model {}

pub enum Msg {}

impl<CTX> Component<CTX> for Model{
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
		Model {}
	}
	fn update(&mut self, _msg: Self::Message, _env: &mut Env<CTX, Self>) -> ShouldRender {
		true
	}
}
impl<CTX: 'static> Renderable<CTX, Model> for Model{
	fn view(&self) -> Html<CTX, Self> {
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