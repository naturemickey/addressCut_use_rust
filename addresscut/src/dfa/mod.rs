
mod data_cache;

use std::collections::HashMap;
use std::option::Option;
use std::iter::Iterator;
use self::data_cache::base_data::City;

struct DfaState {
	name:String,
	path:HashMap<char, DfaState>,	
}

impl DfaState {
	fn new(n:String) -> DfaState {
		DfaState{name:n, path:HashMap::new()}
	}
	fn add_path(&mut self, c:char, name:String) -> &mut DfaState {
		self.path.entry(c).or_insert(DfaState::new(name))
	}
	fn is_accepted(&self) -> bool {
		self.name != ""
	}
	fn tran(&self, c:&char) -> Option<&DfaState> {
		self.path.get(c)
	}
}

pub struct Dfa {
	startState:DfaState,
	citys:Vec<City>,
	name_map:HashMap<String, Vec<i32>>
}

impl Dfa {
	fn new(citys:Vec<City>, name_map:HashMap<String, Vec<i32>>) -> Dfa {
		Dfa{startState:DfaState::new("".to_string()), citys:citys, name_map: name_map}
	}
}

pub fn scan() {
	let (citys, name_map) = data_cache::assemble_data();
}