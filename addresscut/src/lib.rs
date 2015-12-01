
pub mod dfa;

use self::dfa::DFA;
use self::dfa::data_cache::base_data::City;
use std::collections::HashMap;
use std::result::Result::{Ok, Err};

pub struct AddressScanner {
	dfa:DFA
}

impl AddressScanner {
	pub fn new() -> AddressScanner {
		AddressScanner{dfa: DFA::new()}
	}
	pub fn scan(&self, s:&str) -> Address {
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
		let vv = break_tree(&tree);
		// print_anvv(&vv);
		let v = choose(&addr_list, vv, 0);
		// for a in v {
		// 	print!("{} ", a.city.name);
		// }
		// println!("===========");
		fix(v, s, &self.dfa)
	}
}

pub struct Address {
	pub province_address :String, // 省 level 1
	pub city_address     :String, // 市 level 2
	pub area_address     :String, // 区 level 3
	pub town_address     :String, // 镇/街道办 level 4
	pub original_address :String,
	pub detail_address   :String,
}

fn fix<'a>(mut v:Vec<&'a AddrNode<'a>>, s:&str, dfa:&DFA) -> Address {
	let mut res = Address{
		province_address : "".to_string(),
		city_address     : "".to_string(),
		area_address     : "".to_string(),
		town_address     : "".to_string(),
		original_address : s.to_string(),
		detail_address   : "".to_string(),
	};
	if v.len() == 0 {
		res.detail_address = s.to_string();
	} else { unsafe {
		let addr = v.get_unchecked(v.len() - 1);
		let last_real_addr = &addr.addr;
		let last_std_addr = &addr.city.name;
		let mut id = addr.city.id;
		while id != 0 {
			let city = &dfa.citys[id - 1];
			match city.lvl {
				1 => res.province_address = city.name.to_string(),
				2 => res.city_address = city.name.to_string(),
				3 => res.area_address = city.name.to_string(),
				4 => res.town_address = city.name.to_string(),
				_ => {}
			}
			id = city.pid;
		}
		if let Some(i) = s.rfind(last_std_addr) {
			res.detail_address = s.slice_unchecked(i, s.len()).to_string();
		} else if let Some(i) = s.rfind(last_real_addr) {
			res.detail_address = s.slice_unchecked(i, s.len()).to_string();
		} else {
			res.detail_address = s.to_string();
		}
	}}
	res
}

fn choose<'a>(addr_list:&Vec<String>, mut vv:Vec<Vec<&'a AddrNode<'a>>>, idx:usize) -> Vec<&'a AddrNode<'a>> {
	if vv.len() == 0 {
		return Vec::new();
	}
	if vv.len() == 1 {
		return vv.remove(0);
	}
	let mut len = vv.len();
	let mut res1:Vec<Vec<&'a AddrNode<'a>>> = Vec::new();
	let mut res1_max_len:usize = 0;
	while len > 0 {
		len -= 1;
		let v = vv.remove(len);
		let mut lvl:i8 = 20;
		let mut st:i8 = 0;
		if v.len() > idx { unsafe {
			let a = v.get_unchecked(idx);
			if (lvl >= 3) && (a.city.lvl < lvl) {
				st = 1;
				lvl = a.city.lvl;
				if res1_max_len < v.len() {
					res1_max_len = v.len();
				}
			} else if (a.city.lvl == lvl) || ((lvl <= 2) && (a.city.lvl <= 2)) {
				st = 2;
				if res1_max_len < v.len() {
					res1_max_len = v.len();
				}
			}
		}}
		if st == 1 {
			res1.clear();
			res1.push(v);
		} else if st == 2 {
			res1.push(v);
		}
	}
	if res1.len() == 1 {
		return res1.remove(0);
	}
	let mut is_std = false;
	let mut to_recu = false;
	let mut res2:Vec<Vec<&'a AddrNode<'a>>> = Vec::new();
	let mut len = res1.len();
	let mut flg:usize = 0;
	let mut to_push:Vec<usize> = Vec::new();
	while len > 0 { unsafe {
		len -= 1;
		let v = res1.remove(len);
		let a = v.get_unchecked(idx);
		if a.city.lvl < 2 {
			flg = 1;
			to_recu = res1_max_len > idx + 1;
			break;
		}
		if is_std {
			if a.addr.len() == a.city.name.len() {
				to_recu = to_recu || (v.len() > idx + 1);
				to_push.push(len);
			}
		} else {
			if a.addr.len() == a.city.name.len() {
				res2.clear();
				to_recu = false;
				is_std = true;
			}
			to_push.push(len);
			to_recu = to_recu || (v.len() > idx + 1);
		}
	}}
	if flg == 1 {
		res2 = res1;
	} else {
		for idx in to_push {
			res2.push(res1.remove(idx));
		}
	}
	if to_recu {
		return choose(addr_list, res2, idx + 1);
	}
	if res2.len() == 0 {
		return Vec::new();
	}
	if res2.len() == 1 {
		return res2.remove(0);
	}

	let mut res = Vec::new();
	let mut len = res2.len();
	while len > 0 {
		len -= 1;
		let v:Vec<&'a AddrNode<'a>> = res2.remove(len);
		if res.len() == 0 {
			res = v;
		} else {
			let mut flg:i8 = 0;
			unsafe {
				let a1 = res.get_unchecked(idx);
				let a2 = v.get_unchecked(idx);
				let mut i1:usize = 10000;
				let mut i2:usize = 10000;
				if let Ok(i) = addr_list.binary_search(&a1.addr.to_string()) {
					i1 = i;
				}
				if let Ok(i) = addr_list.binary_search(&a2.addr.to_string()) {
					i2 = i;
				}
				if i1 < i2 {
					flg = 1;
				}
			}
			if flg == 1 {
				res = v;
			}
		}
	}
	return res;
}

fn print_anvv<'a>(vv:&Vec<Vec<&'a AddrNode<'a>>>) {
	for v in vv {
		for an in v {
			print!("{}_", an.addr);
		}
		println!("");
	}
}

fn break_tree<'a>(tree:&'a Vec<AddrNode<'a>>) -> Vec<Vec<&'a AddrNode<'a>>> {
	let mut res:Vec<Vec<&'a AddrNode<'a>>> = Vec::new();
	for i in 0 .. tree.len() { unsafe {
		let an = tree.get_unchecked(i);
		if an.city.lvl < 4 {
			let mut vv = break_tree(&an.children);
			if vv.len() == 0 {
				let mut v:Vec<&'a AddrNode<'a>> = Vec::new();
				v.push(an);
				res.push(v);
			} else {
				for v in &mut vv {
					let mut v1:Vec<&'a AddrNode<'a>> = Vec::with_capacity(1 + v.len());
					v1.push(an);
					for n in v {
						v1.push(n);
					}
					res.push(v1);
				}
			}
		}
	}}
	res
}

fn make_tree<'a>(addr_list:&'a Vec<String>, citys:&'a Vec<City>, name_map:&HashMap<String, Vec<usize>>) -> Vec<AddrNode<'a>> {
	if addr_list.len() == 0 {
		Vec::new()
	} else {
		let mut res = Vec::new();
		for addr in addr_list {
			if let Some(ids) = name_map.get(addr) {
				for id in ids { unsafe {
					let city = citys.get_unchecked(id - 1);
					add_2_tree(&mut res, city, citys, addr);
				}}
			}
		}
		res
	}
}

fn add_2_tree<'a>(tree:&mut Vec<AddrNode<'a>>, city:&'a City, citys:&'a Vec<City>, addr:&'a str) {
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
					add_2_tree(&mut node.children, city, citys, addr);
				}
				break;
			}
		}
	}}
	if replace_idx >= 0 { unsafe {
		let an = AddrNode{city:city, addr:addr, children:Vec::with_capacity(1)};
		tree.push(an);
		let c = tree.swap_remove(replace_idx as usize);
		tree.get_unchecked_mut(replace_idx as usize).children.push(c);
	}}
	if !has_relationship {
		tree.push(AddrNode{city:city, addr:addr, children:Vec::new()});
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
	addr:&'a str,
	children:Vec<AddrNode<'a>>
}
