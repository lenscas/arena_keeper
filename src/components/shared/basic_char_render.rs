use crate::components::shared::health_bar::HealthBarProps;
use crate::components::shared::health_bar::health_bar;
use crate::classes::character::Character;
use yew::prelude::*;


pub fn basic_char_render<Comp: Component>(character : Character) -> Html<Comp> {
	let image = character.get_image();
	html! {
		<div class="row",>
			<div class="col-md-3",>
				<img class="img-fluid",alt={image.1}, src={image.0},/>
			</div>
			<div class="col",>
				<h5>{character.name.clone()}</h5>
				<div class="row",>
					<div class="col-md-9",>
						{
							health_bar(
								HealthBarProps {
									max: character.max_health,
									current:character.cur_health,
									break_yellow:50,
									break_red:20,
								}
							)
						}
					</div>
					<div class="col",>
						{"HP"}
					</div>
				</div>
			</div>
		</div>
	}
}