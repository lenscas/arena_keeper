use crate::components::game::Game;
use yew::prelude::*;


pub fn index<Comp: Component>() -> Html<Comp> {
	html! {
		<div class="container-fluid", id="mainPage",>
			<div class=("row","h-100"),>
				<canvas id="canvas", style="width:100%;height:100%", width="500",height="265",></canvas>
				<Game:/>
			</div>
		</div>
	}
}