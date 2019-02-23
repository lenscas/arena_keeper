use crate::agents::character_agent::MaybeCharWithId;
use crate::agents::character_agent::CharWithId;

pub fn ensure_all_some_chars(maybe_chars : &[MaybeCharWithId] ) -> Option<Vec<CharWithId>> {
	let mut vec : Vec<CharWithId> = Vec::new();
	for v in maybe_chars.iter() {
		if let Some(character) =  &v.character {
			vec.push(CharWithId::create(v.id, character.clone()));
		} else {
			return None;
		}
	}
	Some(vec)

}
pub fn ensure_all_some<T>(vec : &[Option<T>]) -> Option<Vec<T>> where
	T : Clone
{
	let mut new_vec : Vec<T> = Vec::new();
	for v in vec.iter() {
		if let Some(new_v) = v {
			new_vec.push(new_v.clone());
		} else {
			return None;
		}
	}
	Some(new_vec)
}