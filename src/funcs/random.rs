use stdweb::Value;
use stdweb::unstable::TryInto;

pub fn get_between(min:i32, max:i32) -> i32 {
	assert!(min < max, format!("Min ({}) is not smaller than max ({})",min,max));
	let v : Value = js!{
		return randomIntFromInterval(@{min},@{max})
	};
	let v : i32 = v.try_into().expect("Something wend wrong generating random number");
	v
}

pub fn get_max(max:i32) -> i32 {
	let v : Value = js!{
		return getMax(@{max})
	};
	let v : i32 = v.try_into().expect("Something wend wrong generating random number");
	v
}