#[derive(bart_derive::BartDisplay)]
#[template = "examples/hello_world.html"]
struct HelloWorld<'a> {
    name: &'a str,
}

fn main() {
    print!("{}", &HelloWorld { name: "World" });
}
