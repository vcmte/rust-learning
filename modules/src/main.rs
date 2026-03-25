pub mod garden;

use garden::vegetables::Asparagus;
use std::collections::HashMap;
// use std::fmt;
// use std::io;
use std::fmt::Result;
use std::io::Result as ResultIO;
use std::{cmp::Ordering, char};
use std::io::{self, Write};
use std::collections::*;

fn main() {
    let plant = Asparagus {};

    println!("I'm growing {plant:?}!");

    // ####################################################

    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Usage of same-name structs/enums/etc. imported from parent modules

/*
fn function1() -> fmt::Result {
    // --snip--
}
*/

/*
fn function2() -> io::Result<()> {
    // --snip--
}
*/

// ####################################################

/*
fn function2() -> ResultIO<()> {
    // --snip--
}
*/