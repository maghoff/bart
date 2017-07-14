#[derive(Debug, PartialEq, Eq)]
pub struct Name<'a> {
    pub leading_dots: u32,
    pub segments: Vec<&'a str>,
    pub function_call: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SectionType {
    // {{#section}}
    Iteration,

    // {{^section}}
    NegativeIteration,

    // {{#section?}}
    Conditional,

    // {{^section?}}
    NegativeConditional,

    // {{#section.}}
    Scope,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Literal(&'a str),
    Interpolation(Name<'a>),
    UnescapedInterpolation(Name<'a>),
    SectionOpener(SectionType, Name<'a>),
    SectionCloser(Name<'a>),
    PartialInclude(&'a str, Name<'a>),
}

#[cfg(test)]
pub fn simple_name(name: &'static str) -> Name<'static> {
    Name {
        leading_dots: 0,
        segments: vec![name],
        function_call: false,
    }
}
