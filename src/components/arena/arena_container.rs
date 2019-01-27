use crate::components::arena::fight_item::FightItem;
use crate::components::arena::fight_creation_char_display::SideCharDisplay;
use crate::agents::fight_agent;
use stdweb::traits::IEvent;
use yew::prelude::*;
use std::str::FromStr;

pub struct Arena {
	fights : Vec<fight_agent::FightId>,
	worker: Box<Bridge<fight_agent::Worker>>,
	lethal_chance: i32
}


pub enum Msg {
	UpdateFights(fight_agent::Response),
	UpdateLethalChance(i32),
	CreateFight(SubmitEvent)
}
#[derive(PartialEq, Clone)]
pub struct Props {}

impl<'a> Default for Props {
	fn default() -> Self {
		Props {
		}
	}
}
impl Component for Arena
{
	type Message = Msg;
	type Properties = Props;
	fn create(_props: Self::Properties, mut link:  ComponentLink<Self>) -> Self {
		let callback = link.send_back(|res| Msg::UpdateFights(res));
		let worker = fight_agent::Worker::bridge(callback);

		let mut ar = Arena {
			fights : Vec::new(),
			lethal_chance : 0,
			worker
		};
		ar.worker.send(fight_agent::Request::GetAllFights);
		ar
	}
	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::UpdateFights(res) => {
				match res {
					fight_agent::Response::UpdateFightList(list) => {
						self.fights = list;
						true
					},
					_ => {
						unreachable!();
					}
				}
			},
			Msg::UpdateLethalChance(lethal_chance) => {
				self.lethal_chance = lethal_chance;
				true
			},
			Msg::CreateFight(event) => {
				event.prevent_default();
				self.worker.send(fight_agent::Request::CreateFight(self.lethal_chance));
				true
			}
		}
	}
	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		true
	}
}
impl Renderable<Arena> for Arena
{
	fn view(&self) -> Html<Self> {
		html! {
			<div class=("card","h-100"),>
				<div class=("card-header","h-10"),>
					<h1>{"Fights"}</h1>
				</div>
				<ul class=("list-group","list-item-flush","h-90", "scrollBar"),>
					{self.render_fight_planner()}
					{for(self.fights).iter().map(
						|v| html! {
							<FightItem: fight_id=v ,/>
						}
					)}
				</ul>
			</div>
		}
	}
}
impl Arena {
	fn render_fight_planner(&self) -> Html<Self>{
		html! {
			<li class="list-group-item",>
				<form onsubmit=|e|Msg::CreateFight(e), >
					<div class="row",>
						<div class="col-md-4",>
							<SideCharDisplay: is_left=true, />
						</div>
						<div class="col-md-4",style="text-align:center",>
							<h2>{"Safety rules"}</h2>
							<div class="row",>
								<div class="col-md-2",>
									<h3>{"All"}</h3>
								</div>
								<div class="col",>
									<input
										id="ex1",
										class="custom-range",
										data-slider-id="ex1Slider",
										type="range",
										min="0",
										max="20",
										data-slider-step="1",
										data-slider-value="14",
										oninput=|v|Msg::UpdateLethalChance(i32::from_str(v.value.as_str()).unwrap()),
										value=self.lethal_chance,
									/>
								</div>
								<div class="col-md-3",>
									<h3>{"None"}</h3>
								</div>
							</div>
						</div>
						<div class="col-md-4",>
							<SideCharDisplay: is_left=false, />
						</div>
					</div>
					<div class=("row","justify-content-end"),>
						<div class="col-md-1",>
							<button class="btn btn-success",>
								{"Plan"}
							</button>
						</div>
					</div>
				</form>
			</li>
		}
	}
}