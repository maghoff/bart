#![feature(proc_macro)]
#![feature(specialization)]
#[macro_use] extern crate stach_derive;

mod display_html_safe;

struct Nested {
    a: i32,
}

#[derive(StacheDisplay)]
#[template = "src/template.html"]
struct Greeting<'a> {
    name: &'a str,
    age: i32,
    good: bool,
    stuff: Vec<i32>,
    nested: Nested,
}

fn main() {
    print!("{}", &Greeting {
        name: "Brille<tag attr=\"value\" attr2='value'>War & peas",
        age: 32,
        good: true,
        stuff: vec![],
        nested: Nested { a: 10 },
    });

    print!("{}", &Greeting {
        name: "Kinasjakk",
        age: 32,
        good: false,
        stuff: vec![1,2,3],
        nested: Nested { a: 20 },
    });
}
