const TAG_OPENER: &'static str = "{{";
const TAG_CLOSER: &'static str = "}}";

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Mismatch
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

fn bart_tag<'a>(input: &'a str) -> Result<(&'a str, Token<'a>), Error> {
    let input = consume(input, TAG_OPENER)?;

    enum TagType { // TODO Refactor into functions instead?
        Interpolation,
        SectionOpener,
        SectionCloser,
    };

    let (input, tag_type) = match input.chars().next() {
        Some('#') => (&input[1..], TagType::SectionOpener),
        Some('/') => (&input[1..], TagType::SectionCloser),
        Some(_) => (input, TagType::Interpolation),
        None => return Err(Error::Mismatch),
    };

    let (input, name) = match input.find(TAG_CLOSER) {
        Some(index) => Ok((&input[index..], &input[0..index])),
        None => Err(Error::Mismatch)
    }?;

    let input = consume(input, TAG_CLOSER)?;

    Ok((input, match tag_type {
        TagType::Interpolation => Token::Interpolation(name),
        TagType::SectionOpener => Token::SectionOpener(name),
        TagType::SectionCloser => Token::SectionCloser(name),
    }))
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
