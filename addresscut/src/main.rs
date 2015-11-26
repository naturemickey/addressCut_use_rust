
extern crate addresscut;

use addresscut::dfa::DFA;

fn main() {
    DFA::new().scan("abc");
}