#![feature(proc_macro)]

extern crate some_dep;

use some_dep::say_hello;

pub fn main() {
    say_hello!();
}
