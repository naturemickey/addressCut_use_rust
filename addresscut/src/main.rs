
extern crate addresscut;

use addresscut::AddressScanner;
use addresscut::Address;

fn main() {
    let address = AddressScanner::new().scan("广广东省c福田区ef深圳gh");
    println!("{}", &address.province_address);
    println!("{}", &address.city_address);
    println!("{}", &address.area_address);
    println!("{}", &address.town_address);
    println!("{}", &address.original_address);
    println!("{}", &address.detail_address);
}