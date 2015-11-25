
mod base_data;

use std::collections::HashMap;
use std::option::Option::{None, Some};
use std::option::Option;

pub fn im() {
	let ac = base_data::all_citys();
	let mut nameMap:HashMap<String, Vec<i32> > = HashMap::new();
	for c in &ac {
		for name in &c.names {
			nameMap.entry(name.to_string()).or_insert(Vec::new()).push(c.id);
		}
	}
}