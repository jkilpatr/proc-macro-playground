#![feature(proc_macro)]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn say_hello(_input: TokenStream) -> TokenStream {
    "println!(\"Hello, world!\")".parse().unwrap()
}
