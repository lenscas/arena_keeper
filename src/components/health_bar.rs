
use yew::prelude::*;

pub struct HealthBar {
	max : i64,
	current : i64,
	break_yellow : i64,
	break_red : i64
}

pub enum Msg {}
#[derive(PartialEq,Clone)]
pub struct Props {
	pub max : i64,
	pub current : i64,
	pub break_yellow : i64,
	pub break_red : i64
}

impl Default for Props {
	fn default() -> Self {
		Props {
			max : 0,
			current : 0,
			break_yellow : 0,
			break_red : 0,
		}
	}
}

impl<CTX: 'static> Component<CTX> for HealthBar {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
		HealthBar {
			max : props.max,
			current : props.current,
			break_yellow : props.break_yellow,
			break_red : props.break_red
		}
	}
	fn update(&mut self, _msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
		true
	}
	fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
		self.max = props.max;
		self.current = props.current;
		self.break_yellow = props.break_yellow;
		self.break_red = props.break_red;
		true
	}
}

impl<CTX: 'static> Renderable<CTX, HealthBar> for HealthBar {
	fn view(&self) -> Html<CTX, Self> {
		let amount =  self.current * 100 / self.max;// * 100;
		let width = "width: ".to_owned() + (&amount.to_string()) + "%";
		let color :  &str ;
		if amount > self.break_yellow {
			color =  "bg-success";
		} else if amount > self.break_red {
			color =  "bg-warning";
		} else {
			color =  "bg-danger";
		}
		html! {
			<div class="progress",>
				<div 
					class=("progress-bar", color),
					role="progressbar",
					style={&width},
					aria-valuenow={&width},
					aria-valuemax="100",
				>
					{self.current.to_string() + "/"+ &self.max.to_string()}
				</div>
			</div>
		}
	}
}