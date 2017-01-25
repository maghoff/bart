extern crate syn;

const TAG_OPENER: &'static str = "{{";
const TAG_CLOSER: &'static str = "}}";

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Mismatch
}

#[derive(Debug, PartialEq, Eq)]
pub struct Name<'a> {
    name: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Literal(&'a str),
    Interpolation(&'a str),
    SectionOpener(&'a str),
    SectionCloser(&'a str),
}

fn consume<'a>(input: &'a str, expected: &str) -> Result<&'a str, Error> {
    match input.starts_with(expected) {
        true => Ok(&input[expected.len()..]),
        false => Err(Error::Mismatch),
    }
}

fn name<'a>(input: &'a str) -> Result<(&'a str, Name<'a>), Error> {
    let end_opt = input.find(TAG_CLOSER);
    // TODO Consider other terminators, like punctuation and whitespace

    let end = end_opt.ok_or(Error::Mismatch)?;

    let name = &input[0..end];

    syn::parse_ident(name).map_err(|_| Error::Mismatch)?;

    Ok((&input[end..], Name { name: name }))
}

fn interpolation<'a>(input: &'a str) -> Result<(&'a str, Token<'a>), Error> {
    let (rest, name) = name(input)?;
    Ok((rest, Token::Interpolation(name.name)))
}

fn section_opener<'a>(input: &'a str) -> Result<(&'a str, Token<'a>), Error> {
    let input = consume(input, "#")?;
    let (rest, name) = name(input)?;
    Ok((rest, Token::SectionOpener(name.name)))
}

fn section_closer<'a>(input: &'a str) -> Result<(&'a str, Token<'a>), Error> {
    let input = consume(input, "/")?;
    let (rest, name) = name(input)?;
    Ok((rest, Token::SectionCloser(name.name)))
}

fn bart_tag<'a>(input: &'a str) -> Result<(&'a str, Token<'a>), Error> {
    let input = consume(input, TAG_OPENER)?;

    let (input, tag) = match input.chars().next() {
        Some('#') => section_opener(input)?,
        Some('/') => section_closer(input)?,
        Some(_) => interpolation(input)?,
        None => return Err(Error::Mismatch),
    };

    let input = consume(input, TAG_CLOSER)?;

    Ok((input, tag))
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
            Ok(("tail", Token::Interpolation("ape"))),
            bart_tag("{{ape}}tail")
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
            Ok(("", Token::SectionOpener("ape"))),
            bart_tag("{{#ape}}")
        );
    }

    #[test]
    fn bart_tag_matches_section_closer() {
        assert_eq!(
            Ok(("", Token::SectionCloser("ape"))),
            bart_tag("{{/ape}}")
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
            Interpolation("a".into()),
            Interpolation("b".into()),
            Interpolation("c".into()),
        ], parsed);
    }

    #[test]
    fn template_with_mixed_content() {
        let parsed = sequence("Hello {{name}}! {{#list}}Welcome{{/list}}").unwrap();
        assert_eq!(vec![
            Literal("Hello "),
            Interpolation("name"),
            Literal("! "),
            SectionOpener("list"),
            Literal("Welcome"),
            SectionCloser("list"),
        ], parsed);
    }
}
