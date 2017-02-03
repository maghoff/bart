use token;

#[derive(Debug, PartialEq, Eq)]
pub enum Ast<'a> {
    Literal(&'a str),
    Interpolation(token::Name<'a>),
    UnescapedInterpolation(token::Name<'a>),
    Sequence(Vec<Ast<'a>>),
    Iteration { name: token::Name<'a>, nested: Box<Ast<'a>> },
    NegativeIteration { name: token::Name<'a>, nested: Box<Ast<'a>> },
    Conditional { name: token::Name<'a>, nested: Box<Ast<'a>> },
    NegativeConditional { name: token::Name<'a>, nested: Box<Ast<'a>> },
    Scope { name: token::Name<'a>, nested: Box<Ast<'a>> },
}
