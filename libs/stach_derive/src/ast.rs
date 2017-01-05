pub enum Ast {
    Sequence(Vec<Ast>),
    Literal(&'static str),
    Interpolation(&'static str),
    UnescapedInterpolation(&'static str),
    Iteration { ident: &'static str, nested: Box<Ast> },
    Conditional { ident: &'static str, nested: Box<Ast> },
    NegativeConditional { ident: &'static str, nested: Box<Ast> },
    Scope { ident: &'static str, nested: Box<Ast> },
}
