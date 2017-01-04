#![feature(proc_macro)]
#![feature(specialization)]
#[macro_use] extern crate stach_derive;

mod display_html_safe;

#[derive(StacheDisplay)]
#[template = "template.mu.html"]
struct Greeting<'a> {
    name: &'a str,
    age: i32,
}

fn main() {
    print!("{}", &Greeting { name: "Brille<tag attr=\"value\" attr2='value'>War & peas", age: 32 });
    print!("{}", &Greeting { name: "Kinasjakk", age: 32 });
}
