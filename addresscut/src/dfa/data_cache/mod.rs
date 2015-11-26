
pub mod base_data;

use std::collections::HashMap;
use self::base_data::City;
//use std::option::Option::{None, Some};

pub fn assemble_data() -> (Vec<City>, HashMap<String, Vec<i32>>) {
	let ac = base_data::all_citys();
	let mut name_map = HashMap::new();
	for c in &ac {
		for name in &c.names {
			name_map.entry(name.to_string()).or_insert(Vec::new()).push(c.id);
		}
	}
	(ac, name_map)
}