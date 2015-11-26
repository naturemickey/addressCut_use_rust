
mod data_cache;

use std::collections::HashMap;
use std::option::Option;
use std::option::Option::{None, Some};
use self::data_cache::base_data::City;

struct DfaState {
	name:String,
	path:HashMap<char, DfaState>,	
}

impl DfaState {
	fn new(n:String) -> DfaState {
		DfaState{name:n, path:HashMap::new()}
	}
	fn add_path(&mut self, c:char, name:&str) -> &mut DfaState {
		self.path.entry(c).or_insert(DfaState::new(name.to_string()))
	}
	fn add_path_by_name(&mut self, name:&str) {
		self.add_path_by_chars(name.chars().collect(), 0, name);
	}
	fn add_path_by_chars(&mut self, chars:Vec<char>, idx:usize, name:&str) {
		if idx < chars.len() { unsafe {
			// todo chars 借用了一次，下面就不能再移动了，要改。
			let c = chars.get_unchecked(idx);
			let state = self.add_path(*c, if idx == chars.len() - 1 {""} else {name});
			state.add_path_by_chars(chars, idx+1, name);
		}}
	}
	fn is_accepted(&self) -> bool {
		self.name != ""
	}
	fn tran(&mut self, c:&char) -> Option<&mut DfaState> {
		self.path.get_mut(c)
	}
}

pub struct DFA {
	startState:DfaState,
	citys:Vec<City>,
	name_map:HashMap<String, Vec<i32>>
}

impl DFA {
	fn new(citys:Vec<City>, name_map:HashMap<String, Vec<i32>>) -> DFA {
		let mut dfa = DFA{startState:DfaState::new("".to_string()), citys:citys, name_map: name_map};
		for name in dfa.name_map.keys() {
			dfa.startState.add_path_by_name(name);
		}
		dfa
	}
}

pub fn scan() {
	let (citys, name_map) = data_cache::assemble_data();
}