use yew::prelude::*;
use stdweb::traits::IEvent;
use crate::agents::router;
use crate::generated::routes::Windows;
pub struct Link {
	router: Box<Bridge<router::Worker>>,
	text: String,
	class:String,
	action: router::Request
}

pub enum Msg {
	HandleRoute,
	Clicked(ClickEvent)
}

#[derive(PartialEq, Clone)]
pub struct Props {
	pub text: String,
	pub class:String,
	pub action : router::Request
}

impl Default for Props {
	fn default() -> Self {
		Props {
			text:String::from(""),
			class:String::from(""),
			action : router::Request::ActivateWindow(Windows::Shop)
		}
	}
}

impl Component for Link {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties,mut link: ComponentLink<Self>) -> Self {
		let callback = link.send_back(|_| Msg::HandleRoute);
		let router = router::Worker::bridge(callback);
		Link {
			text:props.text,
			action:props.action,
			router,
			class:props.class
		}
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::HandleRoute => {
				false
			}
			Msg::Clicked(e) => {
				e.prevent_default();
				self.router.send(self.action);
				false
			},
		}
	}
	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.class = props.class;
		self.text = props.text;
		self.action = props.action;
		true
	}
}

impl Renderable<Link> for Link {
	fn view(&self) -> Html<Self> {
		html! {
			<a class={self.class.to_owned()}, href={String::from("#") }, onclick=|e|Msg::Clicked(e),>{self.text.to_owned()}</a>
		}
	}
}
