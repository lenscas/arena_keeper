use std::collections::LinkedList;
use yew::prelude::*;

use crate::components::new_character::NewCharacter;
use crate::components::character_display::CharacterDisplay;
use crate::components::character;
pub struct CharacterList {
	characters : LinkedList<character::Character>
}

pub enum Msg {
	OnBuy(character::Character)
}
impl<CTX> Component<CTX> for CharacterList
{
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
		CharacterList {
			characters : LinkedList::new()
		}
	}
	fn update(&mut self, msg: Self::Message, _env: &mut Env<CTX, Self>) -> ShouldRender {
		match msg {
			Msg::OnBuy(new_char) => {
				println!("Did buy!");
				self.characters.push_front(new_char);
				true
			}
		}
	}
}
impl<CTX: 'static> Renderable<CTX, CharacterList> for CharacterList
{
	fn view(&self) -> Html<CTX, Self> {
		html! {
			<div class="card",>
				<div class="card-header",>
					<NewCharacter: on_buy=Msg::OnBuy,/>
				</div>
				<ul class=("list-group","list-item-flush"),>
					{
						for(self.characters).iter().map(
							|character| html! {
								<CharacterDisplay: is_character=character,/>
							}
						)
					}
				</ul>
			</div>
		}
	}
}

impl CharacterList {}
