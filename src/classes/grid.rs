pub struct Grid {
	pub cell_size : u8
}
impl Grid {
	pub fn snap_to_grid(&self,x : f64, y :f64) -> (f64,f64) {
		let fsize = self.as_float();
		(
			x - (x % fsize) ,
			y - (y % fsize)
		)
	}
	pub fn as_float(&self) -> f64 {
		f64::from(self.cell_size)
	}
	pub fn as_int(&self) ->i32 {
		i32::from(self.cell_size)
	}
	pub fn as_usize(&self) -> usize {
		usize::from(self.cell_size)
	}
}