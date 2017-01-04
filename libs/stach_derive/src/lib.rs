#![feature(proc_macro)]
#![feature(proc_macro_lib)]
#![recursion_limit = "128"]

#[macro_use] extern crate quote;

extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

// Yield mock generated code for template
//     Hello, {{name}} ({{age}})
#[proc_macro_derive(StacheDisplay)]
pub fn stache_display(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();

    let type_name = &ast.ident;
    let generics = &ast.generics;

    let gen = quote! {
        impl #generics std::fmt::Display for #type_name #generics {
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
    };

    gen.parse().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
