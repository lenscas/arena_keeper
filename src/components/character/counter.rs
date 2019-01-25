use yew::prelude::*;

pub struct Counter {
	clicked: i64,
	to_add: i64,
}

pub enum Msg {
	Clicked,
	Rerender,
}

#[derive(PartialEq, Clone)]
pub struct Props {
	pub to_add: i64,
}

impl Default for Props {
	fn default() -> Self {
		Props { to_add: 1 }
	}
}

impl Component for Counter {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
		Counter {
			clicked: 0,
			to_add: props.to_add,
		}
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Clicked => {
				self.clicked = self.clicked + self.to_add;
				false
			}
			Msg::Rerender => true,
		}
	}
}

impl Renderable<Counter> for Counter {
	fn view(&self) -> Html<Self> {
		html! {
			<div>
				<button class="btn", class="btn-success", onclick=|_| Msg::Clicked,>{"Click me!"}</button>
				<button onclick=|_| Msg::Rerender,>{"Rerender"}</button>
				<p>{self.clicked}</p>
				<h1>{"I GOT RENDERED!"}</h1>
			</div>
		}
	}
}
