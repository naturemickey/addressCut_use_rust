
mod base_data;

pub fn im() {
	let ac = base_data::all_citys();
	for c in ac {
		println!("{}", c.to_string());
	}
}