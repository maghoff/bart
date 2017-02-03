extern crate num;
extern crate syn;

use token::*;

const TAG_OPENER: &'static str = "{{";
const TAG_CLOSER: &'static str = "}}";
const UNESCAPED_TAG_CLOSER: &'static str = "}}}";

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Mismatch
}

fn consume<'a>(input: &'a str, expected: &str) -> Result<&'a str, Error> {
    match input.starts_with(expected) {
        true => Ok(&input[expected.len()..]),
        false => Err(Error::Mismatch),
    }
}

fn not_dot(ch: char) -> bool {
    ch != '.'
}

fn name<'a>(input: &'a str) -> Result<(&'a str, Name<'a>), Error> {
    let leading_dots = input.find(not_dot).ok_or(Error::Mismatch)?;
    let input = &input[leading_dots..];

    let mut segments = vec![];
    for segment in input.split('.') {
        syn::parse_ident(segment).map_err(|_| Error::Mismatch)?;
        segments.push(segment);
    }

    Ok((&input[0..0], Name {
        leading_dots: num::cast::cast(leading_dots).unwrap(),
        segments: segments,
    }))
}

fn at_end(input: &str) -> Result<(), Error> {
    match input.len() {
        0 => Ok(()),
        _ => Err(Error::Mismatch),
    }
}

fn interpolation<'a>(input: &'a str) -> Result<Token<'a>, Error> {
    let (rest, name) = name(input)?;
    at_end(rest)?;
    Ok(Token::Interpolation(name))
}

fn unescaped_interpolation<'a>(input: &'a str) -> Result<Token<'a>, Error> {
    let input = consume(input, "{")?;
    let (rest, name) = name(input)?;
    at_end(rest)?;
    Ok(Token::UnescapedInterpolation(name))
}

fn section_opener<'a>(input: &'a str) -> Result<Token<'a>, Error> {
    let input = consume(input, "#")?;
    let (rest, name) = name(input)?;
    at_end(rest)?;
    Ok(Token::SectionOpener(SectionType::Iteration, name))
}

fn section_closer<'a>(input: &'a str) -> Result<Token<'a>, Error> {
    let input = consume(input, "/")?;
    let (rest, name) = name(input)?;
    at_end(rest)?;
    Ok(Token::SectionCloser(name))
}

fn bart_tag<'a>(input: &'a str) -> Result<(&'a str, Token<'a>), Error> {
    let input = consume(input, TAG_OPENER)?;

    let peek = input.chars().next();
    let tag_closer = match peek {
        Some('{') => UNESCAPED_TAG_CLOSER,
        _ => TAG_CLOSER,
    };

    let end = input.find(tag_closer).ok_or(Error::Mismatch)?;
    let tag_meat = &input[..end];
    let rest = &input[end + tag_closer.len()..];

    let tag = match peek {
        Some('#') => section_opener(tag_meat)?,
        Some('/') => section_closer(tag_meat)?,
        Some('{') => unescaped_interpolation(tag_meat)?,
        Some(_) => interpolation(tag_meat)?,
        None => return Err(Error::Mismatch),
    };

    Ok((rest, tag))
}

fn literal_text<'a>(input: &'a str) -> Result<(&'a str, Option<Token<'a>>), Error> {
    match input.find(TAG_OPENER) {
        Some(0) => Ok((input, None)),
        Some(index) => Ok((
            &input[index..],
            Some(Token::Literal(&input[0..index]))
        )),
        None => Ok((
            "",
            match input.len() {
                0 => None,
                _ => Some(Token::Literal(input))
            }
        ))
    }
}

pub fn sequence<'a>(mut input: &'a str) -> Result<Vec<Token<'a>>, Error> {
    let mut seq = vec![];

    loop {
        let (rest, literal_opt) = literal_text(input)?;

        if let Some(literal) = literal_opt {
            seq.push(literal);
        }

        if rest.is_empty() {
            break;
        }

        let (rest, tag) = bart_tag(rest)?;
        seq.push(tag);

        input = rest;
    }

    Ok(seq)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Token::*;

    #[test]
    fn consume_matches() {
        assert_eq!(Ok("ape}}"), consume("{{ape}}", "{{"));
    }

    #[test]
    fn consume_mismatches() {
        assert_eq!(Err(Error::Mismatch), consume("{{ape}}", "{a"));
    }

    #[test]
    fn bart_tag_matches() {
        assert_eq!(
            Ok(("tail", Token::Interpolation(simple_name("ape")))),
            bart_tag("{{ape}}tail")
        );
    }

    #[test]
    fn bart_tag_matches_name_with_underscore() {
        assert_eq!(
            Ok(("tail", Token::Interpolation(simple_name("ape_katt")))),
            bart_tag("{{ape_katt}}tail")
        );
    }

    #[test]
    fn bart_tag_mismatches() {
        assert_eq!(
            Err(Error::Mismatch),
            bart_tag("head{{ape}}")
        );
    }

    #[test]
    fn bart_tag_must_be_closed() {
        assert_eq!(
            Err(Error::Mismatch),
            bart_tag("{{ape")
        );
    }

    #[test]
    fn bart_tag_matches_section_opener() {
        assert_eq!(
            Ok(("", Token::SectionOpener(SectionType::Iteration, simple_name("ape")))),
            bart_tag("{{#ape}}")
        );
    }

    #[test]
    fn bart_tag_matches_section_closer() {
        assert_eq!(
            Ok(("", Token::SectionCloser(simple_name("ape")))),
            bart_tag("{{/ape}}")
        );
    }

    #[test]
    fn bart_tag_matches_unescaped_interpolation() {
        assert_eq!(
            Ok(("", Token::UnescapedInterpolation(simple_name("ape")))),
            bart_tag("{{{ape}}}")
        );
    }

    #[test]
    fn error_on_invalid_tag() {
        let res = bart_tag("{{+ape}}");
        assert!(res.is_err());
    }

    #[test]
    fn literal_reads_until_tag() {
        assert_eq!(
            Ok(("{{ape}}", Some(Token::Literal("head")))),
            literal_text("head{{ape}}")
        );
    }

    #[test]
    fn literal_reads_until_end() {
        assert_eq!(
            Ok(("", Some(Token::Literal("head{ape}")))),
            literal_text("head{ape}")
        );
    }

    #[test]
    fn literal_returns_none_at_tag() {
        assert_eq!(
            Ok(("{{ape}}", None)),
            literal_text("{{ape}}")
        );
    }

    #[test]
    fn literal_returns_none_at_end() {
        assert_eq!(
            Ok(("", None)),
            literal_text("")
        );
    }

    #[test]
    fn template_with_tightly_packed_tags() {
        let parsed = sequence("{{a}}{{b}}{{c}}").unwrap();
        assert_eq!(vec![
            Interpolation(simple_name("a")),
            Interpolation(simple_name("b")),
            Interpolation(simple_name("c")),
        ], parsed);
    }

    #[test]
    fn template_with_mixed_content() {
        let parsed = sequence("Hello {{name}}! {{#list}}Welcome{{/list}}").unwrap();
        assert_eq!(vec![
            Literal("Hello "),
            Interpolation(simple_name("name")),
            Literal("! "),
            SectionOpener(SectionType::Iteration, simple_name("list")),
            Literal("Welcome"),
            SectionCloser(simple_name("list")),
        ], parsed);
    }

    #[test]
    fn tags_with_leading_dots() {
        let parsed = sequence("{{.a}}{{..b}}{{...c}}").unwrap();
        assert_eq!(vec![
            Interpolation(Name { leading_dots: 1, segments: vec!["a"] }),
            Interpolation(Name { leading_dots: 2, segments: vec!["b"] }),
            Interpolation(Name { leading_dots: 3, segments: vec!["c"] }),
        ], parsed);
    }

    #[test]
    fn tags_with_segmented_names() {
        let parsed = sequence("{{a.b.c}}{{..b.c.d}}").unwrap();
        assert_eq!(vec![
            Interpolation(Name { leading_dots: 0, segments: vec!["a", "b", "c"] }),
            Interpolation(Name { leading_dots: 2, segments: vec!["b", "c", "d"] }),
        ], parsed);
    }
}
