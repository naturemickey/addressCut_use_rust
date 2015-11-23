
extern crate addresscut;

use std::fs::File;
use std::io::prelude::*;

macro_rules! try1 {
    ($expr:expr) => (match $expr {
        $crate::result::Result::Ok(val) => val,
        $crate::result::Result::Err(err) => {
            return $crate::result::Result::Err($crate::convert::From::from(err))
        }
    })
}

fn main() {
	let mut f = try!(File::open("citybasedata.config"));
	let mut s = String::new();
	try!(f.read_to_string(&mut s));
	println!("{}", s);
	println!("address cut");
}
