use stdweb::Value;
use stdweb::unstable::TryInto;
pub fn getMax(max:i32) -> i32 {
	let v : Value = js!{
		return getMax(@{max})
	};
	let v : i32 = v.try_into().expect("Something wend wrong generating random number");
	v
}