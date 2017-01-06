use std::fmt::{self, Display, Write};

struct EscapingWriter<'a> {
    inner: &'a mut Write
}

impl<'a> EscapingWriter<'a> {
    fn new(inner: &'a mut Write) -> EscapingWriter<'a> {
        EscapingWriter { inner: inner }
    }
}

impl<'a> Write for EscapingWriter<'a> {
    fn write_str(&mut self, buf: &str) -> fmt::Result {
        // Sneaky use of String::split, capturing the separator:

        let mut separator = '_';
        for part in buf.split(|x| { separator = x; (x == '<') || (x == '&') || (x == '\'') || (x == '"') }) {
            self.inner.write_str(part)?;

            match separator {
                '<' => self.inner.write_str("&lt;"),
                '&' => self.inner.write_str("&amp;"),
                '\'' => self.inner.write_str("&apos;"),
                '"' => self.inner.write_str("&quot;"),
                _ => Ok(()),
            }?;
        }

        Ok(())
    }
}

pub trait DisplayHtmlSafe {
    fn safe_fmt(&self, &mut fmt::Formatter) -> fmt::Result;
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
    }
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

    struct Fake<'a> { text: &'a str }
    impl<'a> Display for Fake<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.text.safe_fmt(f)
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(
            " &lt; &amp; &quot; &apos; ",
            format!("{}", Fake { text: " < & \" ' " })
        );
    }

    #[test]
    fn it_handles_tight_packed_string() {
        assert_eq!(
            "&lt;&amp;&quot;&apos;",
            format!("{}", Fake { text: "<&\"'" })
        );
    }
}