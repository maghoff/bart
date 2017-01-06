use ast::*;
use ast::Ast::*;
use nom::*;
use std::io;
use std::io::prelude::*;
use std::fs::File;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) { from() }
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

named!(name(&str) -> Name,
    chain!(
        dots: many0!(tag!(".")) ~
        name:
            opt!(
                recognize!(
                    pair!(
                        alphanumeric,
                        many0!(
                            pair!(
                                tag!("."),
                                alphanumeric
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

named!(stache_tag(&str) -> Ast,
    alt!(
        scope |
        unescaped_interpolation_tag |
        interpolation_tag
    )
);

named!(text_or_tag(&str) -> Ast,
    alt!(
        stache_tag |
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

pub fn parse_file(filename: &str) -> Result<Ast, Error> {
    let mut f = File::open(filename)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

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
    fn test() {
        kake("src/template.mu.html").unwrap();
    }
}
