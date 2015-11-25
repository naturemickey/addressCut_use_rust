
mod base_data;

use std::collections::HashMap;
use std::option::Option::{None, Some};
use std::option::Option;

fn insert_to_name_map(nameMap:&mut HashMap<String, Vec<i32> >, name:&str, id:i32) {
	match nameMap.get_mut(name) {
		None => {
			let mut v = Vec::new();
			v.push(id);
			nameMap.insert(name.to_string(), v);
		}
		Some(v) => {
			v.push(id);
		}
	}
}

pub fn im() {
	let ac = base_data::all_citys();
	let mut nameMap:HashMap<String, Vec<i32> > = HashMap::new();
	for c in &ac {
		for name in &c.names {
			insert_to_name_map(&mut nameMap, name, c.id);
		}
	}
}