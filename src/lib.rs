/*!

Bart is a compile time templating language inspired by
[Mustache](https://mustache.github.io/mustache.5.html). It plays to Rust's
strengths by statically compiling the template into efficient code and
performing full variable resolution and type checking at compile time.

Cargo dependencies
==================
To use Bart, add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
bart = "0.1.0"
bart_derive = "0.1.0"
```

Example
=======
Given the template file `hello_world.html`:

```text
Hello {{name}}
```

We can write the following program:

```
#[macro_use] extern crate bart_derive;

#[derive(BartDisplay)]
#[template = "examples/hello_world.html"]
struct HelloWorld<'a> {
    name: &'a str,
}

fn main() {
    print!("{}", &HelloWorld { name: "World" });
#   assert_eq!("Hello World\n", format!("{}", &HelloWorld { name: "World" }));
}
```

*/

#![cfg_attr(feature = "specialization", feature(specialization))]

#[macro_use] extern crate nom;

mod display_html_safe;

// With specialization, DisplayHtmlSafe could be something that the
// user wants to deal with. But specialization is still unstable.
#[doc(hidden)]
pub use display_html_safe::DisplayHtmlSafe;

mod conditional;
pub use conditional::Conditional;

mod negative_iterator;
pub use negative_iterator::NegativeIterator;
