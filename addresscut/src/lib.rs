
pub mod dfa;

use self::dfa::DFA;

pub struct AddressScanner {
	dfa:DFA
}

impl AddressScanner {
	pub fn new() -> AddressScanner {
		AddressScanner{dfa: DFA::new()}
	}
	pub fn scan(&self, s:&str) -> Vec<String> {
		// self.dfa.print_states();
		let chars = s.chars().collect();
		let mut addrList = self.dfa.scan(&chars);
		
		let mut is_first = true;
		let mut res:Vec<String> = Vec::with_capacity(addrList.len() + 1);
		if addrList.len() > 0 { unsafe {
			let s = addrList.get_unchecked(0);
			if s == "北京" {
				res.push("北京市".to_string());
			} else if s == "上海" {
				res.push("上海市".to_string());
			} else if s == "天津" {
				res.push("天津市".to_string());
			} else if s == "重庆" {
				res.push("重庆市".to_string());
			}
			res.extend(addrList.iter().cloned());
		}}
		res
	}
}