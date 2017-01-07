#[derive(Debug, PartialEq, Eq)]
pub struct Name {
    pub dots: usize,
    pub name: Option<String>,
}

impl Name {
    pub fn resolve(&self, scope_depth: usize) -> String {
        let root = match self.dots {
            0 => "self".to_owned(),
            x => format!("_s{}", scope_depth.checked_sub(x).expect("Too many dots")),
        };
        match self.name {
            Some(ref name) => format!("{}.{}", root, name),
            None => root,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ast {
    Sequence(Vec<Ast>),
    Literal(String),
    Interpolation(Name),
    UnescapedInterpolation(Name),
    Iteration { name: Name, nested: Box<Ast> },
    Conditional { name: Name, nested: Box<Ast> },
    NegativeConditional { name: Name, nested: Box<Ast> },
    Scope { name: Name, nested: Box<Ast> },
}
