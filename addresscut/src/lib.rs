
pub mod dfa;

use self::dfa::DFA;
use self::dfa::data_cache::base_data::City;
use std::collections::HashMap;
use std::option::Option;
use std::option::Option::{None, Some};

pub struct AddressScanner {
	dfa:DFA
}

impl AddressScanner {
	pub fn new() -> AddressScanner {
		AddressScanner{dfa: DFA::new()}
	}
	pub fn scan(&self, s:&str) -> Vec<String> {
		let chars = s.chars().collect();
		let addr_list = self.dfa.scan(&chars);

		let mut res:Vec<String> = Vec::with_capacity(addr_list.len() + 1);
		if addr_list.len() > 0 { unsafe {
			let s = addr_list.get_unchecked(0);
			if s == "北京" {
				res.push("北京市".to_string());
			} else if s == "上海" {
				res.push("上海市".to_string());
			} else if s == "天津" {
				res.push("天津市".to_string());
			} else if s == "重庆" {
				res.push("重庆市".to_string());
			}
			res.extend(addr_list.iter().cloned());
		}}
		let citys:&Vec<City> = &self.dfa.citys;
		let name_map:&HashMap<String, Vec<i32>> = &self.dfa.name_map;
		let tree = make_tree(&addr_list, citys, name_map);
		res
	}
}

fn make_tree<'a>(addr_list:&Vec<String>, citys:&Vec<City>, name_map:&HashMap<String, Vec<i32>>) -> Vec<AddrNode<'a>> {
	if addr_list.len() == 0 {
		Vec::new()
	} else {
		let mut res = Vec::new();
		res
	}
}

struct AddrNode<'a> {
	city:&'a City,
	children:Vec<AddrNode<'a>>
}