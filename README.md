[![Build Status](https://travis-ci.org/maghoff/bart.svg?branch=master)](https://travis-ci.org/maghoff/bart)

Bart is a compile time templating language for [Rust](https://www.rust-lang.org/en-US/) inspired by [Mustache](https://mustache.github.io/mustache.5.html). It plays to Rust's strengths by statically compiling the template into efficient code and performing full variable resolution and type checking at compile time.

Cargo dependencies
==================
To use Bart, add these dependencies to your `Cargo.toml`:

    [dependencies]
    bart = "0.1.4"
    bart_derive = "0.1.4"

Example
=======
Given the template file `hello_world.html`:

    Hello {{name}}

We can write the following program:

    #[macro_use] extern crate bart_derive;

    #[derive(BartDisplay)]
    #[template = "hello_world.html"]
    struct HelloWorld<'a> {
        name: &'a str,
    }

    fn main() {
        print!("{}", &HelloWorld { name: "World" });
    }

To compile this example program, you need to add both `bart` and `bart_derive` as dependencies in your `Cargo.toml`.

Running this program will output

    Hello World

You can run this example by cloning this repository and executing `cargo run --example hello_world`.

Line by line
------------
    #[macro_use] extern crate bart_derive;

The programmer interface to Bart is the procedural macro defined in the `bart_derive` crate, which implements support for `#[derive(BartDisplay)]`. It must be added as a dependency in your `Cargo.toml` and referenced like above. `bart_derive` generates code which is dependent on the `bart` crate, so you also need to pull this in as a dependency.

    #[derive(BartDisplay)]

Use `bart_derive` to generate an `impl` of the [`Display`][Display] trait based on the template and struct below.

    #[template = "hello_world.html"]

`bart_derive` will read `hello_world.html` and use it to generate the template rendering code. The given file name is relative to your crate root, so, for example, you have to specify `#[template = "src/hello_world.html"]` if you want your template to reside in the `src/` directory.

It is also possible to specify the template inline with `template_string`: `#[template_string = "Hello {{name}}"]`.

    struct HelloWorld<'a> {
        name: &'a str,
    }

Values to be interpolated in the template will be resolved from the given `struct`. In this case `{{name}}` would be resolved to the `name` field of this struct. Fields to be interpolated must implement the [`Display`][Display] trait.

    fn main() {
        print!("{}", &HelloWorld { name: "World" });
    }

As noted above, `bart_derive` has now generated an `impl` of [`Display`][Display] for `HelloWorld`. This means we can pass instances of `HelloWorld` to `print!`, `write!`, `format!` and so on. The template is rendered with the supplied data, generating `Hello World` to standard output.

Language reference
==================
The Bart templating language is inspired by Mustache. (Bart is the Norwegian word for Mustache.)

The input is reproduced verbatim except for tags. Tags start with `{{` and end with `}}`.

Interpolation
-------------
The simplest tag is the interpolation tag, which contains a data reference. For the template `Hello {{name}}`, `{{name}}` is recognized as an interpolation tag and `name` is resolved as a field on the given `struct`. This field must implement the [`Display`][Display] trait. It is possible to use `.` to refer to fields in nested `struct`s; `{{name.surname}}`.

Interpolation tags are HTML escaped, so for the template `Hello {{name}}`, if `{{name}}` is `Bobby <tags>`, the output will be `Hello Bobby &lt;tags>`.

Verbatim/unescaped interpolation
--------------------------------
It is also useful to be able to deliberately include HTML content unescaped. Use triple-tags, `{{{`&hellip;`}}}`, for this: `Hello {{{name}}}` would render `Hello Bobby <tags>` if `name` were `Bobby <tags>`.

Iteration
---------
It is possible to iterate over anything that implements [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html):

    <ul>
    {{#values}}
        <li>{{.}}</li>
    {{/values}}
    </ul>

Use `{{.}}` to refer to the current value. For example, if `values` were a `Vec<i32>`, `{{.}}` would refer to each of the contained `i32` values in turn. When iterating over a set of structures, use a `.` prefix to refer to members:

    <ul>
    {{#people}}
        <li>{{.name}} ({{.age}})</li>
    {{/people}}
    </ul>

It can be useful to take advantage of the `IntoIterator` implementations on `Option` and `Result` to use them in Bart iterations.

Scoping
-------
Similar to iteration, it is possible to enter a scope for a variable, by specifying a trailing dot:

    {{#person.}}
        {{.name}} ({{.age}})
    {{/person}}

It is also possible to fully qualify each reference:

    {{person.name}} ({{person.age}})

When in a nested scope, use multiple leading dots to step out:

    {{#department.}}
        {{#head.}}
            {{.name}}, head of the {{..name}} department.
        {{/head}}
    {{/department}}

Unqualified names, that is, names without leading dots, will always be resolved in the topmost scope.

The same scoping rules apply to iteration scopes.


[Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
