
pub mod dfa;

use self::dfa::DFA;
use self::dfa::data_cache::base_data::City;
use std::collections::HashMap;

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
		let name_map:&HashMap<String, Vec<usize>> = &self.dfa.name_map;
		let tree = make_tree(&addr_list, citys, name_map);
		res
	}
}

fn make_tree<'a>(addr_list:&Vec<String>, citys:&'a Vec<City>, name_map:&HashMap<String, Vec<usize>>) -> Vec<AddrNode<'a>> {
	if addr_list.len() == 0 {
		Vec::new()
	} else {
		let mut res = Vec::new();
		for addr in addr_list {
			if let Some(ids) = name_map.get(addr) {
				for id in ids { unsafe {
					let city = citys.get_unchecked(id - 1);
					add_2_tree(&mut res, city, citys);
				}}
			}
		}
		res
	}
}

fn add_2_tree<'a>(tree:&mut Vec<AddrNode<'a>>, city:&'a City, citys:&'a Vec<City>) {
	let mut has_relationship = false;
	let mut replace_idx:i32 = -1;
	for i in 0 .. tree.len() { unsafe {
		let node = tree.get_unchecked_mut(i);
		if node.city.id != city.id {
			let relationship = get_relationship(city, node.city, citys);
			if relationship != 0 {
				has_relationship = true;
				if relationship > 0 {
					replace_idx = i as i32;
				} else {
					add_2_tree(&mut node.children, city, citys);
				}
				break;
			}
		}
	}}
	if replace_idx >= 0 { unsafe {
		let mut an = AddrNode{city:city, children:Vec::with_capacity(1)};
		tree.push(an);
		let c = tree.swap_remove(replace_idx as usize);
		tree.get_unchecked_mut(replace_idx as usize).children.push(c);
	}}
	if !has_relationship {
		tree.push(AddrNode{city:city, children:Vec::new()});
	}
}

fn get_relationship(ct1:&City, ct2:&City, citys:&Vec<City>) -> i8 {
	if ct1.lvl == ct2.lvl {
		return 0;
	} if ct1.lvl > ct2.lvl {
		return -1 * get_relationship(ct2, ct1, citys);
	}
	let mut ct = ct2;
	while (ct1.lvl < ct.lvl) && (ct.pid != 0) { unsafe {
		let ctp = citys.get_unchecked(ct.pid - 1);
		if ctp.id == ct1.id {
			return 1;
		}
		ct = ctp;
	}}
	return 0;
}

struct AddrNode<'a> {
	city:&'a City,
	children:Vec<AddrNode<'a>>
}
