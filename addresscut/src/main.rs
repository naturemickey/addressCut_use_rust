
extern crate addresscut;

use addresscut::AddressScanner;

fn main() {
    let v = AddressScanner::new().scan("广广东省cd广州广州市ef深圳gh");
    for a in v {
        println!("{}", a);
    }
}