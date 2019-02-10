use crate::components::arena::arena_container::Arena;
use crate::components::character::character_list::CharacterList;
use yew::prelude::*;


pub fn index<Comp: Component>() -> Html<Comp> {
	html! {
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
	}
}