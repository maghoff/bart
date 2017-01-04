use nom::*;
use std::io;
use std::io::prelude::*;
use std::fs::File;

quick_error! {
    #[derive(Debug)]
    enum Error {
        Io(err: io::Error) { from() }
        UnexpectedEOF
        Nom(err: ErrorKind) { from() }
    }
}

#[derive(Debug)]
struct TextAndTag<'a> {
    text: &'a str,
    tag: &'a str,
}

fn brille(template: &str) -> IResult<&str, TextAndTag> {
    chain!(template,
        text: take_until_and_consume!("{{") ~
        tag: take_until_and_consume!("}}"),
        || {
            println!("{:?}", &text);
            println!("{:?}", &tag);
            TextAndTag { text: text, tag: tag }
        }
    )
}

fn kake(filename: &str) -> Result<(), Error> {
    let mut f = File::open(filename)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    match brille(&buf) {
        IResult::Done(_, parsed) => {
            println!("Parsed: {:?}", &parsed);
            Ok(())
        }
        IResult::Error(err) => Err(err.into()),
        IResult::Incomplete(_) => Err(Error::UnexpectedEOF),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        kake("src/template.mu.html").unwrap();
    }
}
