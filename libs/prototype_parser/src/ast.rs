#[derive(Debug, PartialEq, Eq)]
pub enum Ast<'a> {
    Literal(&'a str),
    Interpolation(&'a str),
    Sequence(Vec<Ast<'a>>),
    Section { name: &'a str, nested: Box<Ast<'a>> },
}
