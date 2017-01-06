#[macro_use] extern crate nom;
#[macro_use] extern crate quick_error;
#[macro_use] extern crate quote;

extern crate proc_macro;
extern crate syn;

mod ast;
mod parsbart;

use proc_macro::TokenStream;

use ast::Ast;

fn generate(node: Ast, scope_level: usize) -> quote::Tokens {
    use Ast::*;
    match node {
        Sequence(seq) => {
            let items = seq.into_iter().map(|node| generate(node, scope_level));
            quote! { #(#items)* }
        },
        Literal(text) => {
            quote! { f.write_str(#text)?; }
        },
        Interpolation(name) => {
            let name = syn::Ident::new(name.resolve(scope_level));
            quote! { DisplayHtmlSafe::safe_fmt(&#name, f)?; }
        },
        UnescapedInterpolation(name) => {
            let name = syn::Ident::new(name.resolve(scope_level));
            quote! { Display::fmt(&#name, f)?; }
        },
        Iteration { name, nested } => {
            let name = syn::Ident::new(name.resolve(scope_level));
            let scope_variable = syn::Ident::new(format!("_s{}", scope_level));
            let nested_generated = generate(*nested, scope_level + 1);
            quote! {
                for ref #scope_variable in &#name {
                    #nested_generated
                }
            }
        },
        Conditional { name, nested } => {
            let name = syn::Ident::new(name.resolve(scope_level));
            let scope_variable = syn::Ident::new(format!("_s{}", scope_level));
            let nested_generated = generate(*nested, scope_level + 1);
            quote! {
                if Into::into(#name) {
                    let ref #scope_variable = #name;
                    #nested_generated
                }
            }
        },
        NegativeConditional { name, nested } => {
            let name = syn::Ident::new(name.resolve(scope_level));
            let scope_variable = syn::Ident::new(format!("_s{}", scope_level));
            let nested_generated = generate(*nested, scope_level + 1);
            quote! {
                if Into::<bool>::into(#name) == false {
                    let ref #scope_variable = #name;
                    #nested_generated
                }
            }
        },
        Scope { name, nested } => {
            let name = syn::Ident::new(name.resolve(scope_level));
            let scope_variable = syn::Ident::new(format!("_s{}", scope_level));
            let nested_generated = generate(*nested, scope_level + 1);
            quote! {
                {
                    let ref #scope_variable = #name;
                    #nested_generated
                }
            }
        },
    }
}

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

    let parsed = parsbart::parse_file(template).unwrap();
    let generated = generate(parsed, 1);

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use ::std::fmt::Display;
                use display_html_safe::DisplayHtmlSafe;
                let ref _s0 = self;

                #generated

                Ok(())
            }
        }
    };

    gen.parse().unwrap()
}
