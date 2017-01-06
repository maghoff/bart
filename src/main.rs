#![cfg_attr(feature = "specialization", feature(specialization))]

#[macro_use] extern crate stach_derive;

#[derive(StacheDisplay)]
#[template = "src/null-template.html"]
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
    println!("{}", &Greeting {
        name: "Brille<tag attr=\"value\" attr2='value'>War & peas",
        age: 32,
        good: true,
        stuff: vec![],
        nested: Nested { a: 10 },
    });

    println!("{}", &Greeting {
        name: "Kinasjakk",
        age: 32,
        good: false,
        stuff: vec![1,2,3],
        nested: Nested { a: 20 },
    });
}
