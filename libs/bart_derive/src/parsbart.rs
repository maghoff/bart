use ast::*;
use ast::Ast::*;
use nom::*;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        UnexpectedEOF
        Nom(err: ErrorKind) { from() }
    }
}

enum ScopeType {
    Iteration,
    Conditional,
    Scope,
}

struct OpenScopeTag {
    pub name: Name,
    pub scope_type: ScopeType,
}

struct CloseScopeTag {
    pub name: Name,
}

fn identifier(input: &str) -> IResult<&str, &str> {
  let input_length = input.input_len();
  if input_length == 0 {
    return IResult::Incomplete(Needed::Unknown);
  }

  for (idx, item) in input.iter_indices() {
    if !(item.is_alphanum() || item == '_') {
      if idx == 0 {
        return IResult::Error(error_position!(ErrorKind::AlphaNumeric, input))
      } else {
        return IResult::Done(input.slice(idx..), input.slice(0..idx))
      }
    }
  }
  IResult::Done(input.slice(input_length..), input)
}

named!(name(&str) -> Name,
    chain!(
        dots: many0!(tag!(".")) ~
        name:
            opt!(
                recognize!(
                    pair!(
                        identifier,
                        many0!(
                            pair!(
                                tag!("."),
                                identifier
                            )
                        )
                    )
                )
            ),
        || Name {
            dots: dots.len(),
            name: name.map(|x| x.to_owned()),
        }
    )
);

named!(open_scope_tag(&str) -> OpenScopeTag,
    chain!(
        tag!("{{#") ~
        name: name ~
        operator: opt!(alt!(tag!("?") | tag!("."))) ~
        tag!("}}"),
        || OpenScopeTag {
            name: name,
            scope_type: match operator {
                Some("?") => ScopeType::Conditional,
                Some(".") => ScopeType::Scope,
                _ => ScopeType::Iteration
            }
        }
    )
);

named!(close_scope_tag(&str) -> CloseScopeTag,
    chain!(
        tag!("{{/") ~
        name: name ~
        tag!("}}"),
        || CloseScopeTag { name: name }
    )
);

named!(scope(&str) -> Ast,
    chain!(
        open: open_scope_tag ~
        seq: sequence ~
        close: close_scope_tag,
        move || {
            assert_eq!(open.name.dots, close.name.dots);
            assert_eq!(open.name.name, close.name.name);

            let nested = Box::new(seq);

            match open.scope_type {
                ScopeType::Iteration => Iteration {
                    name: open.name,
                    nested: nested,
                },
                ScopeType::Conditional => Conditional {
                    name: open.name,
                    nested: nested,
                },
                ScopeType::Scope => Scope {
                    name: open.name,
                    nested: nested,
                },
            }
        }
    )
);

named!(unescaped_interpolation_tag(&str) -> Ast,
    chain!(
        tag!("{{{") ~
        name: name ~
        tag!("}}}"),
        || UnescapedInterpolation(name)
    )
);

named!(interpolation_tag(&str) -> Ast,
    chain!(
        tag!("{{") ~
        name: name ~
        tag!("}}"),
        || Interpolation(name)
    )
);

named!(bart_tag(&str) -> Ast,
    alt!(
        scope |
        unescaped_interpolation_tag |
        interpolation_tag
    )
);

named!(text_or_tag(&str) -> Ast,
    alt!(
        bart_tag |
        map_res!(
            take_until!("{{"),
            |x: &str| match x.len() {
                0 => Err(()),
                _ => Ok(Literal(x.to_owned()))
            }
        )
    )
);

named!(sequence(&str) -> Ast,
    map!(
        many0!(complete!(text_or_tag)),
        |x| Sequence(x)
    )
);

named!(template_file(&str) -> Ast,
    chain!(
        main: sequence ~
        tail: rest_s,
        || Sequence(vec![main, Literal(tail.to_owned())])
    )
);

pub fn parse_str(buf: &str) -> Result<Ast, Error> {
    match template_file(&buf) {
        IResult::Done(_, parsed) => Ok(parsed),
        IResult::Error(err) => Err(err.into()),
        IResult::Incomplete(_) => Err(Error::UnexpectedEOF),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_simple_stuff_correctly() {
        assert_eq!(
            parse_str("Hello {{lol}}").unwrap(),
            Sequence(vec![
                Sequence(vec![
                    Literal("Hello ".to_owned()),
                    Interpolation(Name { dots: 0, name: Some("lol".to_owned()) })
                ]),
                Literal("".to_owned())
            ])
        );
    }

    #[test]
    fn it_parses_literal_text_only() {
        assert_eq!(
            parse_str("Hello").unwrap(),
            Sequence(vec![
                Sequence(vec![]),
                Literal("Hello".to_owned())
            ])
        );
    }

    #[test]
    fn it_parses_trailing_text() {
        assert_eq!(
            parse_str("Hello {{name}}!").unwrap(),
            Sequence(vec![
                Sequence(vec![
                    Literal("Hello ".to_owned()),
                    Interpolation(Name { dots: 0, name: Some("name".to_owned()) })
                ]),
                Literal("!".to_owned())
            ])
        );
    }

    #[test]
    fn it_accepts_underscore_in_identifiers() {
        assert_eq!(
            parse_str("Hello {{your_name}}!").unwrap(),
            Sequence(vec![
                Sequence(vec![
                    Literal("Hello ".to_owned()),
                    Interpolation(Name { dots: 0, name: Some("your_name".to_owned()) })
                ]),
                Literal("!".to_owned())
            ])
        );
    }
}
