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
impl Component for CharacterList
{
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		CharacterList {
			characters : LinkedList::new()
		}
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::OnBuy(new_char) => {
				println!("Did buy!");
				self.characters.push_front(new_char);
				true
			}
		}
	}
}
impl Renderable<CharacterList> for CharacterList
{
	fn view(&self) -> Html<Self> {
		html! {
			<div class=("card","h-100"),>
				<div class=("card-header","h-10"),>
					<NewCharacter: on_buy=Msg::OnBuy,/>
				</div>
				<ul class=("list-group","list-item-flush","h-90", "scrollBar"),>
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
