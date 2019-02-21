use crate::agents::character_agent::MaybeCharWithId;

pub struct FightOutcome {
	pub is_done : bool,
	pub chars : Vec<MaybeCharWithId>,
	pub earned_money : i64
}