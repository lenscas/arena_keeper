use crate::classes::character::Character;
use crate::agents::character_agent::CharacterId;

pub struct FightOutcome {
	pub is_done : bool,
	pub chars : (
		(CharacterId,Character),
		(CharacterId,Character),
	),
	pub earned_money : i64
}