#![feature(proc_macro)]
#![feature(proc_macro_lib)]

extern crate proc_macro;
//extern crate syn;

use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro_derive(Stach)]
pub fn stach(_input: TokenStream) -> TokenStream {
    // Yield mock generated code for template
    //     Hello, {{name}} ({{age}})

    TokenStream::from_str(r#"
impl<'a> std::fmt::Display for Greeting<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use display_html_safe::DisplayHtmlSafe;

        f.write_str("Hello, ")?;
        DisplayHtmlSafe::fmt(&self.name, f)?;
        f.write_str(" (")?;
        DisplayHtmlSafe::fmt(&self.age, f)?;
        f.write_str(")\n")?;
        Ok(())
    }
}
"#).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
