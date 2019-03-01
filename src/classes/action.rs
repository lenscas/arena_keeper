use crate::agents::character_agent::CharacterId;

pub struct Action {
	pub on : CharacterId,
	pub accuracy : i32,
	pub damage : i32,
	pub strength : i32,
	pub spectacle_delta : i32
}