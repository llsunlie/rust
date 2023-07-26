use std::{self, io, collections::HashMap};
use std::collections::*;
use rand::Rng;

mod one_mod;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    
    let secret_number = rand::thread_rng().gen_range(1, 101);

    one_mod::child_mod::func1();
}
