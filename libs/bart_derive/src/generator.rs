use ast;
use syn;
use token;
use quote;

fn resolve(name: &token::Name, scope_depth: u32) -> String {
    let root = match name.leading_dots {
        0 => "_s0".to_owned(),
        x => format!("_s{}", scope_depth.checked_sub(x).expect("Too many dots")),
    };

    let mut buf = root;
    for ref segment in &name.segments {
        buf.push('.');
        buf.push_str(segment);
    }

    buf
}

pub fn generate(node: ast::Ast, scope_level: u32) -> quote::Tokens {
    use ast::Ast::*;
    match node {
        Sequence(seq) => {
            let items = seq.into_iter().map(|node| generate(node, scope_level));
            quote! { #(#items)* }
        },
        Literal(text) => {
            quote! { f.write_str(#text)?; }
        },
        Interpolation(name) => {
            let name = syn::Ident::new(resolve(&name, scope_level));
            quote! { _bart::DisplayHtmlSafe::safe_fmt(&#name, f)?; }
        },
        UnescapedInterpolation(name) => {
            let name = syn::Ident::new(resolve(&name, scope_level));
            quote! { ::std::fmt::Display::fmt(&#name, f)?; }
        },
        Iteration { name, nested } => {
            let name = syn::Ident::new(resolve(&name, scope_level));
            let scope_variable = syn::Ident::new(format!("_s{}", scope_level));
            let nested_generated = generate(*nested, scope_level + 1);
            quote! {
                for ref #scope_variable in (&#name).into_iter() {
                    #nested_generated
                }
            }
        },
        NegativeIteration { name, nested } => {
            let name = syn::Ident::new(resolve(&name, scope_level));
            let scope_variable = syn::Ident::new(format!("_s{}", scope_level));
            let nested_generated = generate(*nested, scope_level + 1);
            quote! {
                for ref #scope_variable in _bart::NegativeIterator::neg_iter(&#name) {
                    #nested_generated
                }
            }
        },
        Conditional { name, nested } => {
            let name = syn::Ident::new(resolve(&name, scope_level));
            let scope_variable = syn::Ident::new(format!("_s{}", scope_level));
            let nested_generated = generate(*nested, scope_level + 1);
            quote! {
                if _bart::Conditional::val(&#name) {
                    let ref #scope_variable = #name;
                    #nested_generated
                }
            }
        },
        NegativeConditional { name, nested } => {
            let name = syn::Ident::new(resolve(&name, scope_level));
            let scope_variable = syn::Ident::new(format!("_s{}", scope_level));
            let nested_generated = generate(*nested, scope_level + 1);
            quote! {
                if !_bart::Conditional::val(&#name) {
                    let ref #scope_variable = #name;
                    #nested_generated
                }
            }
        },
        Scope { name, nested } => {
            let name = syn::Ident::new(resolve(&name, scope_level));
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
