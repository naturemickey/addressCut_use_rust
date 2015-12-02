
extern crate addresscut;
extern crate time;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use addresscut::AddressScanner;
use addresscut::Address;
use std::ops::Sub;


fn main() {
    let tm1 = time::now();
    let scanner = AddressScanner::new();
    let tm2 = time::now();
    println!("construct scanner use {} ms.", (tm2.sub(tm1).num_milliseconds()));
    let address = scanner.scan("江西抚州市南昌大学抚州医学分院12级全科2班");
    let tm3 = time::now();
    println!("scan an address use {} ms.", (tm3.sub(tm2).num_milliseconds()));

    println!("province_address : {}", &address.province_address);
    println!("city_address     : {}", &address.city_address);
    println!("area_address     : {}", &address.area_address);
    println!("town_address     : {}", &address.town_address);
    println!("original_address : {}", &address.original_address);
    println!("detail_address   : {}", &address.detail_address);


    let tm4 = time::now();
    let path = Path::new("测试地址.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {}
    }

	let lines:Vec<&str> = s.lines().collect();
    let tm5 = time::now();
    println!("读取所有测试地址消耗时长 {} ms.", (tm5.sub(tm4).num_milliseconds()));
    
    let mut addr_list = Vec::with_capacity(250000);
    for s in lines {
        // println!("{}", s);
        addr_list.push(scanner.scan(s));
    }
    let tm5 = time::now();
    println!("识别所有测试地址消耗时长 {} ms.", (tm5.sub(tm4).num_milliseconds()));
}
