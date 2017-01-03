#![feature(proc_macro)]
#![feature(specialization)]
#[macro_use] extern crate nom;
#[macro_use] extern crate quick_error;
#[macro_use] extern crate stach_derive;

mod parsbart;
mod display_html_safe;

#[derive(Stach)]
struct Greeting<'a> {
    name: &'a str,
    age: i32,
}

fn main() {
    print!("{}", &Greeting { name: "Brille<tag attr=\"value\" attr2='value'>War & peas", age: 32 });
    print!("{}", &Greeting { name: "Kinasjakk", age: 32 });
}
