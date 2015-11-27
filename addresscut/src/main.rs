
extern crate addresscut;

use addresscut::dfa::DFA;

fn main() {
    let dfa = DFA::new();
    // dfa.print_states();
    let v = dfa.scan("广广东省cd广州广州市ef深圳gh");
    for a in v {
        println!("{}", a);
    }
}