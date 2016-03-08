
pub mod data_cache;

use std::collections::HashMap;
use std::option::Option;
use self::data_cache::base_data::City;

struct DfaState {
	name:String,
	path:HashMap<char, DfaState>,	
}

impl DfaState {
	fn new(n:String) -> DfaState {
		DfaState{name:n, path:HashMap::default()}
	}
	fn add_path(&mut self, c:char, name:&str) -> &mut DfaState {
		let mut res = self.path.entry(c).or_insert(DfaState::new(name.to_string()));
		if res.name == "" && name != "" {
			res.name = name.to_string(); 
		}
		res
	}
	fn add_path_by_name(&mut self, name:&str) {
		let chars:Vec<char> = name.chars().collect();
		self.add_path_by_chars(&chars, 0, name);
	}
	fn add_path_by_chars(&mut self, chars:&Vec<char>, idx:usize, name:&str) {
		if idx < chars.len() { unsafe {
			let c = chars.get_unchecked(idx);
			if idx == chars.len() - 1 {
				self.add_path(*c, name);
			} else {
				let state = self.add_path(*c, "");
				state.add_path_by_chars(chars, idx+1, name);
			}
		}}
	}
	fn is_accepted(&self) -> bool {
		self.name != ""
	}
	fn tran(&self, c:&char) -> Option<&DfaState> {
		self.path.get(c)
	}
	fn to_vv_string(&self) -> Vec<Vec<String>> {
		let mut vv:Vec<Vec<String>> = Vec::new();
		let mut v = Vec::new();
		if self.name == "" {
			v.push("~".to_string());
		} else {
			v.push(self.name.to_string());
		}

		vv.push(v);
		for (c, s) in &self.path {
			let mut v2 = Vec::new();
			v2.push("".to_string()); 
			let mut cs = String::new();
			cs.push(*c);
			v2.push(cs);
			vv.push(v2);
			let svv = s.to_vv_string();
			for sv in svv {
				let mut sv2 = Vec::new();
				sv2.push("".to_string());
				sv2.push("".to_string());
				sv2.extend(sv.iter().cloned());
				// for s in sv {
				// 	sv2.push(s.to_string());
				// }
				vv.push(sv2);
			}
		}
		vv
	}
}

pub struct DFA {
	start_state:DfaState,
	pub citys:Vec<City>,
	pub name_map:HashMap<String, Vec<usize>>
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
	pub fn print_states(&self) {
		println!("++++++++++++++++");
		for v in self.start_state.to_vv_string() {
			for s in v {
				print!("{}\t", s);
			}
			println!("");
		};
		println!("++++++++++++++++");
	}
	pub fn scan(&self, chars:&Vec<char>) -> Vec<String> {
		let mut res = Vec::new();
		self.scan_recur(chars, 0, 0, 0, &self.start_state, &self.start_state, &mut res);
		res
	}
	fn scan_recur(&self,
		chars:&Vec<char>, from_idx:usize, current_idx:usize, currect_accepted_idx:usize,
		current_state:&DfaState, current_accepted:&DfaState, res:&mut Vec<String>) {
		let len = chars.len();
		if current_idx < len { unsafe {
			let ch = chars.get_unchecked(current_idx);
			// println!("{}", ch);
			match current_state.tran(&ch) {
				None => if (current_accepted as *const DfaState) != (&self.start_state as * const DfaState) {
					if !res.contains(&current_accepted.name) {
						res.push(current_accepted.name.to_string());
					}
					self.scan_recur(chars, currect_accepted_idx + 1, currect_accepted_idx+1, 0, &self.start_state, &self.start_state, res);
				} else {
					self.scan_recur(chars, from_idx + 1, from_idx + 1, currect_accepted_idx, &self.start_state, current_accepted, res);
				},
				Some(cs) => if current_idx + 1 == len {
					if cs.is_accepted() {
						if !res.contains(&cs.name) {
							res.push(cs.name.to_string());
						}
						self.scan_recur(chars, from_idx, current_idx+1, currect_accepted_idx, &self.start_state, current_accepted, res);
					} if (current_accepted as *const DfaState) != (&self.start_state as * const DfaState) {
						if !res.contains(&current_accepted.name) {
							res.push(current_accepted.name.to_string());
						}
						self.scan_recur(chars, currect_accepted_idx + 1, currect_accepted_idx+1, 0, &self.start_state, &self.start_state, res);
					} else {
						self.scan_recur(chars, from_idx + 1, from_idx, currect_accepted_idx+1, &self.start_state, current_accepted, res);
					}
				} else if cs.is_accepted() {
					self.scan_recur(chars, from_idx, current_idx + 1, current_idx, cs, cs, res);
				} else {
					self.scan_recur(chars, from_idx, current_idx + 1, currect_accepted_idx, cs, current_accepted, res);
				}
			}
		}}
	}
}
