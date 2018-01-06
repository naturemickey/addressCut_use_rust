
extern crate addresscut;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use addresscut::AddressScanner;
use addresscut::Address;
use std::ops::Sub;
use std::result::Result::{Ok, Err};
use std::time::{Duration, Instant};


fn main() {
    let tm1 = Instant::now();
    let scanner = AddressScanner::new();
    let tm2 = Instant::now();
    println!("construct scanner use {:?}.", tm2.duration_since(tm1));
    let address = scanner.scan("江西抚州市南昌大学抚州医学分院12级全科2班");
    let tm3 = Instant::now();
    println!("scan an address use {:?}.", (tm3.duration_since(tm2)));

    println!("province_address : {}", &address.province_address);
    println!("city_address     : {}", &address.city_address);
    println!("area_address     : {}", &address.area_address);
    println!("town_address     : {}", &address.town_address);
    println!("original_address : {}", &address.original_address);
    println!("detail_address   : {}", &address.detail_address);


    let tm4 = Instant::now();
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
    let tm5 = Instant::now();
    println!("读取所有测试地址消耗时长 {:?}.", (tm5.duration_since(tm4)));
    
    let mut addr_list = Vec::with_capacity(250000);
    for i in 0..lines.len() { unsafe {
        // println!("{}", s);
        addr_list.push(scanner.scan(lines.get_unchecked(i)));
    }}
    let tm6 = Instant::now();
    println!("识别所有测试地址消耗时长 {:?}.", (tm6.duration_since(tm5)));
    
    // write_file(addr_list, &lines);
    // let tm7 = time::now();
    // println!("导出所有测试结果消耗时长 {} ms.", (tm7.sub(tm6).num_milliseconds()));
}

macro_rules! try_write {
    ($expr:expr) => (match $expr {
        Ok(_) => {/*do nothing*/},
        Err(why) => panic!("couldn't write : {}", Error::description(&why)),
    })
}

fn write_file(addr_list:Vec<Address>, lines:&Vec<&str>) {
    let path = Path::new("地址切分测试结果.csv");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
    try_write!(file.write_all("\"原始地址\"".as_bytes()));
    try_write!(file.write_all(",\"省(rust)\"".as_bytes()));
    try_write!(file.write_all(",\"市(rust)\"".as_bytes()));
    try_write!(file.write_all(",\"区(rust)\"".as_bytes()));
    try_write!(file.write_all(",\"镇(rust)\"".as_bytes()));
    try_write!(file.write_all(",\"详细地址(rust)\"\n".as_bytes()));
    for i in 0 .. lines.len() { unsafe {
        let a = addr_list.get_unchecked(i);
        let l = lines.get_unchecked(i);
        try_write!(file.write_all((("\"".to_string()) + l + "\"").as_bytes()));
        try_write!(file.write_all(((",\"".to_string()) + &a.province_address + "\"").as_bytes()));
        try_write!(file.write_all(((",\"".to_string()) + &a.city_address + "\"").as_bytes()));
        try_write!(file.write_all(((",\"".to_string()) + &a.area_address + "\"").as_bytes()));
        try_write!(file.write_all(((",\"".to_string()) + &a.town_address + "\"").as_bytes()));
        try_write!(file.write_all("\n".as_bytes()));
    }}

    try_write!(file.sync_all());
}
