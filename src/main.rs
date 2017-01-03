#![feature(specialization)]
#[macro_use] extern crate nom;
#[macro_use] extern crate quick_error;

mod parsbart;
mod display_html_safe;

use std::fmt::{self, Display};

struct Greeting<'a> {
    name: &'a str,
    age: i32,
}

// Mock generated code for template
//     Hello, {{name}} ({{age}})
impl<'a> Display for Greeting<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use display_html_safe::DisplayHtmlSafe;

        f.write_str("Hello, ")?;
        DisplayHtmlSafe::fmt(&self.name, f)?;
        f.write_str(" (")?;
        DisplayHtmlSafe::fmt(&self.age, f)?;
        f.write_str(")\n")?;
        Ok(())
    }
}


fn main() {
    print!("{}", &Greeting { name: "Brille<tag attr=\"value\" attr2='value'>War & peas", age: 32 });
    print!("{}", &Greeting { name: "Kinasjakk", age: 32 });
}
