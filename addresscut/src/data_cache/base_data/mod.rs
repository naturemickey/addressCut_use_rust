

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::option::Option::{None, Some};

pub struct City {
	id:i16,
	pid:i16,
	lvl:i8,
	names:Vec<String>
}

pub fn all_citys() -> Vec<City> {
	let mut citys:Vec<City> = vec![];
	citys.push(City{id:1, pid:0, lvl:1, names:vec!["甘肃".to_string(), "甘肃省".to_string()]});
	citys.push(City{id:2, pid:1, lvl:2, names:vec!["甘南藏族".to_string(), "甘南藏族自治州".to_string(), "甘南".to_string(), "甘南自治州".to_string()]});
	citys.push(City{id:3, pid:2, lvl:3, names:vec!["碌曲".to_string(), "碌曲县".to_string()]});
	citys.push(City{id:4, pid:3, lvl:4, names:vec!["玛艾".to_string(), "玛艾镇".to_string()]});
	citys.push(City{id:5, pid:3, lvl:4, names:vec!["阿拉".to_string(), "阿拉乡".to_string()]});

    let path = Path::new("citybasedata.config");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(cs) => print!("{} size:\n{}", display, cs)
    }

	let lines:Vec<&str> = s.lines().collect();
	for line in lines {
		let ss:Vec<&str> = line.split(',').collect();
		// for s in ss {
		// 	println!("{}", s);
		// }
		for i in 0..ss.len() {
			match ss.get(i) {
				None => println!("error"),
				Some(s) => println!("{}", s)
			}
		}
	}

	citys
}