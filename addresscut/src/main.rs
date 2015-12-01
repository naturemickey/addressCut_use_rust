
extern crate addresscut;

use addresscut::AddressScanner;
use addresscut::Address;

fn main() {
    let scanner = AddressScanner::new();
    let address = scanner.scan("四川遂宁市湖北黄冈市蕲春县湖北省黄冈市蕲春县林业局");
    println!("province_address : {}", &address.province_address);
    println!("city_address     : {}", &address.city_address);
    println!("area_address     : {}", &address.area_address);
    println!("town_address     : {}", &address.town_address);
    println!("original_address : {}", &address.original_address);
    println!("detail_address   : {}", &address.detail_address);
}
