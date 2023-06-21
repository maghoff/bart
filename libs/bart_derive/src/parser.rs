use crate::ast::Ast;
use crate::token::*;
use std::iter::*;

#[derive(Debug)]
pub enum Error<'a> {
    Mismatch {
        expected: &'static str,
        found: Option<Token<'a>>,
    },
}

fn section<'a, T>(token_stream: &mut Peekable<T>) -> Result<Ast<'a>, Error<'a>>
where
    T: Iterator<Item = Token<'a>>,
{
    let (section_type, name) = match token_stream.next() {
        Some(Token::SectionOpener(section_type, name)) => Ok((section_type, name)),
        x => Err(Error::Mismatch {
            expected: "section opener",
            found: x,
        }),
    }?;

    let nested = Box::new(sequence(token_stream)?);

    match token_stream.next() {
        Some(Token::SectionCloser(ref close_name)) if close_name == &name => Ok(()),
        x => Err(Error::Mismatch {
            expected: "section closer",
            found: x,
        }),
    }?;

    Ok(match section_type {
        SectionType::Iteration => Ast::Iteration {
            name: name,
            nested: nested,
        },
        SectionType::NegativeIteration => Ast::NegativeIteration {
            name: name,
            nested: nested,
        },
        SectionType::Conditional => Ast::Conditional {
            name: name,
            nested: nested,
        },
        SectionType::NegativeConditional => Ast::NegativeConditional {
            name: name,
            nested: nested,
        },
        SectionType::Scope => Ast::Scope {
            name: name,
            nested: nested,
        },
    })
}

fn sequence<'a, T>(token_stream: &mut Peekable<T>) -> Result<Ast<'a>, Error<'a>>
where
    T: Iterator<Item = Token<'a>>,
{
    let mut seq: Vec<Ast> = vec![];

    loop {
        seq.push(match token_stream.peek() {
            Some(&Token::Literal(text)) => {
                token_stream.next();
                Ast::Literal(text)
            }
            Some(&Token::Interpolation(_)) => match token_stream.next() {
                Some(Token::Interpolation(name)) => Ast::Interpolation(name),
                _ => panic!("Outer match should guarantee match in inner match"),
            },
            Some(&Token::UnescapedInterpolation(_)) => match token_stream.next() {
                Some(Token::UnescapedInterpolation(name)) => Ast::UnescapedInterpolation(name),
                _ => panic!("Outer match should guarantee match in inner match"),
            },
            Some(&Token::SectionOpener(..)) => section(token_stream)?,
            Some(&Token::PartialInclude(..)) => match token_stream.next() {
                Some(Token::PartialInclude(partial_name, root)) => {
                    Ast::PartialInclude { partial_name, root }
                }
                _ => panic!("Outer match should guarantee match in inner match"),
            },
            _ => break,
        })
    }

    Ok(Ast::Sequence(seq))
}

fn parse_impl<'a, T>(mut token_stream: Peekable<T>) -> Result<Ast<'a>, Error<'a>>
where
    T: Iterator<Item = Token<'a>>,
{
    let seq = sequence(&mut token_stream)?;

    if let Some(x) = token_stream.next() {
        return Err(Error::Mismatch {
            expected: "EOF",
            found: Some(x),
        });
    }

    Ok(seq)
}

pub fn parse<'a, T>(token_stream: T) -> Result<Ast<'a>, Error<'a>>
where
    T: IntoIterator<Item = Token<'a>>,
{
    parse_impl(token_stream.into_iter().peekable())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Ast::Sequence(vec![Ast::Literal("text"),]),
            parse(vec![Token::Literal("text")]).unwrap()
        )
    }

    #[test]
    fn simple_iteration() {
        assert_eq!(
            Ast::Sequence(vec![
                Ast::Literal("text a"),
                Ast::Iteration {
                    name: simple_name("x"),
                    nested: Box::new(Ast::Sequence(vec![Ast::Literal("text b"),]))
                },
                Ast::Literal("text c"),
            ]),
            parse(vec![
                Token::Literal("text a"),
                Token::SectionOpener(SectionType::Iteration, simple_name("x")),
                Token::Literal("text b"),
                Token::SectionCloser(simple_name("x")),
                Token::Literal("text c"),
            ])
            .unwrap()
        )
    }

    #[test]
    fn simple_negative_iteration() {
        assert_eq!(
            Ast::Sequence(vec![
                Ast::Literal("text a"),
                Ast::NegativeIteration {
                    name: simple_name("x"),
                    nested: Box::new(Ast::Sequence(vec![Ast::Literal("text b"),]))
                },
                Ast::Literal("text c"),
            ]),
            parse(vec![
                Token::Literal("text a"),
                Token::SectionOpener(SectionType::NegativeIteration, simple_name("x")),
                Token::Literal("text b"),
                Token::SectionCloser(simple_name("x")),
                Token::Literal("text c"),
            ])
            .unwrap()
        )
    }

    #[test]
    fn simple_conditional() {
        assert_eq!(
            Ast::Sequence(vec![
                Ast::Literal("text a"),
                Ast::Conditional {
                    name: simple_name("x"),
                    nested: Box::new(Ast::Sequence(vec![Ast::Literal("text b"),]))
                },
                Ast::Literal("text c"),
            ]),
            parse(vec![
                Token::Literal("text a"),
                Token::SectionOpener(SectionType::Conditional, simple_name("x")),
                Token::Literal("text b"),
                Token::SectionCloser(simple_name("x")),
                Token::Literal("text c"),
            ])
            .unwrap()
        )
    }

    #[test]
    fn simple_negative_conditional() {
        assert_eq!(
            Ast::Sequence(vec![
                Ast::Literal("text a"),
                Ast::NegativeConditional {
                    name: simple_name("x"),
                    nested: Box::new(Ast::Sequence(vec![Ast::Literal("text b"),]))
                },
                Ast::Literal("text c"),
            ]),
            parse(vec![
                Token::Literal("text a"),
                Token::SectionOpener(SectionType::NegativeConditional, simple_name("x")),
                Token::Literal("text b"),
                Token::SectionCloser(simple_name("x")),
                Token::Literal("text c"),
            ])
            .unwrap()
        )
    }

    #[test]
    fn simple_scope() {
        assert_eq!(
            Ast::Sequence(vec![
                Ast::Literal("text a"),
                Ast::Scope {
                    name: simple_name("x"),
                    nested: Box::new(Ast::Sequence(vec![Ast::Literal("text b"),]))
                },
                Ast::Literal("text c"),
            ]),
            parse(vec![
                Token::Literal("text a"),
                Token::SectionOpener(SectionType::Scope, simple_name("x")),
                Token::Literal("text b"),
                Token::SectionCloser(simple_name("x")),
                Token::Literal("text c"),
            ])
            .unwrap()
        )
    }

    #[test]
    fn section_closer_mismatch() {
        let res = parse(vec![
            Token::SectionOpener(SectionType::Iteration, simple_name("x")),
            Token::SectionCloser(simple_name("y")),
        ]);

        assert!(res.is_err())
    }

    #[test]
    fn understands_unescaped_interpolation() {
        assert_eq!(
            Ast::Sequence(vec![
                Ast::Literal("a"),
                Ast::UnescapedInterpolation(simple_name("b")),
                Ast::Literal("c"),
            ]),
            parse(vec![
                Token::Literal("a"),
                Token::UnescapedInterpolation(simple_name("b")),
                Token::Literal("c"),
            ])
            .unwrap()
        )
    }

    #[test]
    fn partials() {
        assert_eq!(
            Ast::Sequence(vec![Ast::PartialInclude {
                partial_name: "partial",
                root: simple_name("a")
            },]),
            parse(vec![Token::PartialInclude("partial", simple_name("a"))]).unwrap()
        )
    }
}
