
mod base_data;

use std::collections::HashMap;
//use std::option::Option::{None, Some};

pub fn im() {
	let ac = base_data::all_citys();
	let mut name_map = HashMap::new();
	for c in &ac {
		for name in &c.names {
			name_map.entry(name.to_string()).or_insert(Vec::new()).push(c.id);
		}
	}
}