use nom::*;
use std::fmt::{self, Display, Write};

struct EscapingWriter<'a> {
    inner: &'a mut dyn Write,
}

impl<'a> EscapingWriter<'a> {
    fn new(inner: &'a mut dyn Write) -> EscapingWriter<'a> {
        EscapingWriter { inner: inner }
    }
}

named!(part(&str) -> &str,
    alt!(
        map!(tag!("<"), |_| "&lt;" ) |
        map!(tag!("&"), |_| "&amp;" ) |
        map!(tag!("\""), |_| "&quot;" ) |
        map!(tag!("'"), |_| "&apos;" ) |
        is_not!("<&\"'")
    )
);

impl<'a> Write for EscapingWriter<'a> {
    fn write_str(&mut self, buf: &str) -> fmt::Result {
        let mut rest = buf;
        while let IResult::Done(new_rest, parsed) = part(rest) {
            self.inner.write_str(parsed)?;
            rest = new_rest;
        }

        Ok(())
    }
}

pub trait DisplayHtmlSafe {
    fn safe_fmt(&self, _: &mut fmt::Formatter) -> fmt::Result;
}

impl<T: Display> DisplayHtmlSafe for T {
    #[cfg(feature = "specialization")]
    default fn safe_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut escaping_writer = EscapingWriter::new(f);
        write!(&mut escaping_writer, "{}", &self)
    }

    #[cfg(not(feature = "specialization"))]
    fn safe_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut escaping_writer = EscapingWriter::new(f);
        write!(&mut escaping_writer, "{}", &self)
    }
}

macro_rules! display_is_html_safe {
    ($x : ident) => {
        #[cfg(feature = "specialization")]
        impl DisplayHtmlSafe for $x {
            fn safe_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                Display::fmt(&self, f)
            }
        }
    };
}

display_is_html_safe!(u8);
display_is_html_safe!(i8);
display_is_html_safe!(u16);
display_is_html_safe!(i16);
display_is_html_safe!(u32);
display_is_html_safe!(i32);
display_is_html_safe!(u64);
display_is_html_safe!(i64);
display_is_html_safe!(usize);
display_is_html_safe!(isize);

display_is_html_safe!(f32);
display_is_html_safe!(f64);

display_is_html_safe!(bool);

#[cfg(test)]
mod test {
    use super::*;

    struct Fake<'a> {
        text: &'a str,
    }
    impl<'a> Display for Fake<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.text.safe_fmt(f)
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(
            " &lt; &amp; text &quot; &apos; ",
            format!(
                "{}",
                Fake {
                    text: " < & text \" ' "
                }
            )
        );
    }

    #[test]
    fn it_handles_tight_packed_string() {
        assert_eq!(
            "&lt;te&amp;&quot;xt&apos;",
            format!("{}", Fake { text: "<te&\"xt'" })
        );
    }
}
