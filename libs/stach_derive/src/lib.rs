#![feature(proc_macro)]
#![feature(proc_macro_lib)]

#[macro_use] extern crate nom;
#[macro_use] extern crate quick_error;
#[macro_use] extern crate quote;

extern crate proc_macro;
extern crate syn;

mod parsbart;

use proc_macro::TokenStream;

enum Ast {
    Sequence(Vec<Ast>),
    Literal(&'static str),
    Interpolation(&'static str),
    UnescapedInterpolation(&'static str),
    Iteration { ident: &'static str, nested: Box<Ast> },
    Conditional { ident: &'static str, nested: Box<Ast> },
    NegativeConditional { ident: &'static str, nested: Box<Ast> },
}

fn generate(node: Ast, scope_level: i32) -> quote::Tokens {
    use Ast::*;
    match node {
        Sequence(seq) => {
            let items = seq.into_iter().map(|node| generate(node, scope_level));
            quote! { #(#items)* }
        },
        Literal(text) => {
            quote! { f.write_str(#text)?; }
        },
        Interpolation(ident) => {
            let ident = syn::Ident::new(ident);
            quote! { DisplayHtmlSafe::safe_fmt(&#ident, f)?; }
        },
        UnescapedInterpolation(ident) => {
            let ident = syn::Ident::new(ident);
            quote! { Display::fmt(&#ident, f)?; }
        },
        Iteration { ident, nested } => {
            let ident = syn::Ident::new(ident);
            let scope_variable = syn::Ident::new(format!("_s{}", scope_level));
            let nested_generated = generate(*nested, scope_level + 1);
            quote! {
                for ref #scope_variable in &#ident {
                    #nested_generated
                }
            }
        },
        Conditional { ident, nested } => {
            let ident = syn::Ident::new(ident);
            let scope_variable = syn::Ident::new(format!("_s{}", scope_level));
            let nested_generated = generate(*nested, scope_level + 1);
            quote! {
                if Into::into(#ident) {
                    let ref #scope_variable = #ident;
                    #nested_generated
                }
            }
        },
        NegativeConditional { ident, nested } => {
            let ident = syn::Ident::new(ident);
            let scope_variable = syn::Ident::new(format!("_s{}", scope_level));
            let nested_generated = generate(*nested, scope_level + 1);
            quote! {
                if Into::<bool>::into(#ident) == false {
                    let ref #scope_variable = #ident;
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

    parsbart::kake(template).unwrap();
    // TODO Use result from parsing

    let mock_parsed = Ast::Sequence(vec![
        Ast::Literal("Hello, "),
        Ast::Interpolation("self.name"),
        Ast::Literal(" ("),
        Ast::Interpolation("self.age"),
        Ast::Literal(")"),
        Ast::Conditional {
            ident: "self.good",
            nested: Box::new(Ast::Literal(" Good boy!")),
        },
        Ast::NegativeConditional {
            ident: "self.good",
            nested: Box::new(Ast::Literal(" BAD!")),
        },
        Ast::Literal("\n"),
        Ast::Iteration {
            ident: "self.stuff",
            nested: Box::new(Ast::Sequence(vec![
                Ast::Literal("<li>"),
                Ast::Interpolation("_s1"),
                Ast::Literal("</li>\n"),
            ]))
        },
        Ast::Literal("Unescaped name: "),
        Ast::UnescapedInterpolation("self.name"),
        Ast::Literal("\n"),
    ]);

    let generated = generate(mock_parsed, 1);

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
