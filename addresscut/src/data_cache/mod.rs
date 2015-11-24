
mod base_data;

use std::collections::HashMap;
use std::option::Option::{None, Some};

pub fn im() {
	let ac = base_data::all_citys();
	let mut nameMap = HashMap::new();
	for c in ac {
		for name in &c.names {
			let mut b = false;
			let mut idvec:&mut Vec<i32> = match nameMap.get_mut(name) {
				None => {
					b = true;
					Vec::new()
				}
				Some(v) => v
			};
			idvec.push(c.id);
			if b {
				nameMap.insert(name.to_string(), idvec);
			}
		}
	}
}