
extern crate addresscut;

use std::fs::File;
use std::io::prelude::*;

fn main() {
	let mut f = try!(File::open("citybasedata.config"));
	let mut s = String::new();
	try!(f.read_to_string(&mut s));
	println!("{}", s);
	println!("address cut");
}