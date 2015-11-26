
mod data_cache;

use std::collections::HashMap;
use std::option::Option;
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
		let chars:Vec<char> = name.chars().collect();
		self.add_path_by_chars(&chars, 0, name);
	}
	fn add_path_by_chars(&mut self, chars:&Vec<char>, idx:usize, name:&str) {
		if idx < chars.len() { unsafe {
			let c = chars.get_unchecked(idx);
			let state = self.add_path(*c, if idx == chars.len() - 1 {""} else {name});
			state.add_path_by_chars(chars, idx+1, name);
		}}
	}
	fn is_accepted(&self) -> bool {
		self.name != ""
	}
	fn tran(&self, c:&char) -> Option<&DfaState> {
		self.path.get(c)
	}
}

pub struct DFA {
	start_state:DfaState,
	citys:Vec<City>,
	name_map:HashMap<String, Vec<i32>>
}

impl DFA {
	pub fn new() -> DFA {
		let (citys, name_map) = data_cache::assemble_data();
		let mut dfa = DFA{start_state:DfaState::new("".to_string()), citys:citys, name_map: name_map};
		for name in dfa.name_map.keys() {
			dfa.start_state.add_path_by_name(name);
		}
		dfa
	}
	pub fn scan(&self, s:&str) -> Vec<String> {
		let res = Vec::new();
		let chars = s.chars().collect();
		self.scan_recur(&chars, 0, 0, 0, &self.start_state, &self.start_state, &res);
		res
	}
	fn scan_recur(&self,
		chars:&Vec<char>, from_idx:usize, current_idx:usize, currect_accepted_idx:usize,
		current_state:&DfaState, current_accepted:&DfaState, res:&Vec<String>) {
		let len = chars.len();
		if (current_idx < len) {unsafe{
			let ch = chars.get_unchecked(current_idx);
			match current_state.tran(&ch) {
				None => {
					
				} 
				Some(cs) => if current_idx + 1 == len {
					
				} else if cs.is_accepted() {
					self.scan_recur(chars, from_idx, current_idx + 1, current_idx, cs, cs, res);
				}
			}
		}}
	}
}
