/*!

Bart is a compile time templating language inspired by
[Mustache](https://mustache.github.io/mustache.5.html). It plays to Rust's
strengths by statically compiling the template into efficient code and
performing full variable resolution and type checking at compile time.

 1. [Cargo dependencies](#cargo-dependencies)
 2. [Example](#example)
    1. [Line-by-line](#line-by-line)

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

To compile this example program, you need to add both `bart` and `bart_derive` as dependencies in your `Cargo.toml`.

Running this program will output

```text
Hello World
```

You can run this example by cloning this repository and executing `cargo run --example hello_world`.

Line by line
------------
```ignore
#[macro_use] extern crate bart_derive;
```

The programmer interface to Bart is the procedural macro defined in the `bart_derive` crate, which implements support for `#[derive(BartDisplay)]`. It must be added as a dependency in your `Cargo.toml` and referenced like above. `bart_derive` generates code which is dependent on the `bart` crate, so you also need to pull this in as a dependency.

```ignore
#[derive(BartDisplay)]
```

Use `bart_derive` to generate an `impl` of the [`Display`][Display] trait based on the template and struct below.

```ignore
#[template = "hello_world.html"]
```

`bart_derive` will read `hello_world.html` and use it to generate the template rendering code. The given file name is relative to your crate root, so, for example, you have to specify `#[template = "src/hello_world.html"]` if you want your template to reside in the `src/` directory.

It is also possible to specify the template inline with `template_string`: `#[template_string = "Hello {{name}}"]`.

```ignore
struct HelloWorld<'a> {
    name: &'a str,
}
```

Values to be interpolated in the template will be resolved from the given `struct`. In this case `{{name}}` would be resolved to the `name` field of this struct. Fields to be interpolated must implement the [`Display`][Display] trait.

```ignore
fn main() {
    print!("{}", &HelloWorld { name: "World" });
}
```

As noted above, `bart_derive` has now generated an `impl` of [`Display`][Display] for `HelloWorld`. This means we can pass instances of `HelloWorld` to `print!`, `write!`, `format!` and so on. The template is rendered with the supplied data, generating `Hello World` to standard output.

[Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html

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
