use token;

#[derive(Debug, PartialEq, Eq)]
pub enum Ast<'a> {
    Literal(&'a str),
    Interpolation(token::Name<'a>),
    UnescapedInterpolation(token::Name<'a>),
    Sequence(Vec<Ast<'a>>),
    Section { name: token::Name<'a>, nested: Box<Ast<'a>> },
}
