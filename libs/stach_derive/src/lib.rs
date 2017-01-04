#![feature(proc_macro)]
#![feature(proc_macro_lib)]
#![recursion_limit = "128"]

#[macro_use] extern crate nom;
#[macro_use] extern crate quick_error;
#[macro_use] extern crate quote;

extern crate proc_macro;
extern crate syn;

mod parsbart;

use proc_macro::TokenStream;

// Yield mock generated code for template
//     Hello, {{name}} ({{age}})
#[proc_macro_derive(StacheDisplay, attributes(template))]
pub fn stache_display(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();

    let template_attr = ast.attrs.iter()
        .find(|&x| x.name() == "template")
        .expect("#[derive(StacheDisplay)] requires #[template = \"...\"]");

    let template = match &template_attr.value {
        &syn::MetaItem::NameValue(_, syn::Lit::Str(ref template, _)) => template,
        _ => panic!("#[derive(StacheDisplay)] requires #[template = \"...\"]")
    };

    let _ = template; // TODO Parse template file

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use display_html_safe::DisplayHtmlSafe;

                f.write_str("Hello, ")?;
                DisplayHtmlSafe::safe_fmt(&self.name, f)?;
                f.write_str(" (")?;
                DisplayHtmlSafe::safe_fmt(&self.age, f)?;
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
