
use yew::prelude::*;


pub struct HealthBarProps {
	pub max : i64,
	pub current : i64,
	pub break_yellow : i64,
	pub break_red : i64
}
pub fn health_bar<Comp: Component>(props : HealthBarProps) -> Html<Comp> {
	let amount =  props.current * 100 / props.max;// * 100;
	let width = "width: ".to_owned() + (&amount.to_string()) + "%";
	let color :  &str ;
	if amount > props.break_yellow {
		color =  "bg-success";
	} else if amount > props.break_red {
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
				{props.current.to_string() + "/"+ &props.max.to_string()}
			</div>
		</div>
	}
}