use std::fmt::{self, Display, Write};

struct Data<'a> {
    name: &'a str,
    age: i32,
}

impl<'a> Display for Data<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt("Hello, ", f)?;
        Display::fmt(&Escape::new(&self.name), f)?;
        Display::fmt(" (", f)?;
        Display::fmt(&self.age, f)?;
        Display::fmt(")\n", f)?;
        Ok(())
    }
}

struct EscapingFormatter<'a> {
    inner: &'a mut Write
}

impl<'a> EscapingFormatter<'a> {
    fn new(inner: &'a mut Write) -> EscapingFormatter<'a> {
        EscapingFormatter { inner: inner }
    }
}

impl<'a> Write for EscapingFormatter<'a> {
    fn write_str(&mut self, buf: &str) -> fmt::Result {
        unimplemented!()
    }
}

struct Escape<T: Display> {
    inner: T
}

impl<T: Display> Escape<T> {
    fn new(inner: T) -> Escape<T> {
        Escape { inner: inner }
    }
}

impl<T: Display> Display for Escape<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut escaping_formatter = EscapingFormatter::new(f);
        write!(&mut escaping_formatter, "{}", &self.inner)
    }
}

fn main() {
    print!("{}", &Data { name: "L<o&l", age: 32 });
}
