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
			<div class="container-fluid",>
				<div class="row",>
					<div class="col-md-3",>
						<CharacterList: />
					</div>
				</div>
			</div>
		}
	}
}