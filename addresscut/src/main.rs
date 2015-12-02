
extern crate addresscut;
extern crate time;

use addresscut::AddressScanner;
use addresscut::Address;
use std::ops::Sub;


fn main() {
    let tm1 = time::now();
    let scanner = AddressScanner::new();
    let tm2 = time::now();
    println!("construct scanner use {} ms.", (tm2.sub(tm1).num_milliseconds()));
    let address = scanner.scan("山东省莱芜市莱城区吕花园美食街，品品香茶业");
    let tm3 = time::now();
    println!("scan an address use {} ms.", (tm3.sub(tm2).num_milliseconds()));
    
    println!("province_address : {}", &address.province_address);
    println!("city_address     : {}", &address.city_address);
    println!("area_address     : {}", &address.area_address);
    println!("town_address     : {}", &address.town_address);
    println!("original_address : {}", &address.original_address);
    println!("detail_address   : {}", &address.detail_address);
}
