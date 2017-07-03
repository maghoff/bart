use ast;
use itertools;
use syn;
use token;
use quote;

pub trait PartialsResolver {
    fn generate_partial(&mut self, partial_name: &str) -> quote::Tokens;
}

fn resolve(name: &token::Name, scope_depth: u32) -> syn::Ident {
    use itertools::Itertools;

    let root = match name.leading_dots {
        0 => "_s0".to_owned(),
        x => {
            let level = scope_depth
                .checked_sub(x)
                .unwrap_or_else(|| {
                    panic!(format!("Too many leading dots ({}) in scope depth of only {}", x, scope_depth));
                });
            format!("_s{}", level)
        },
    };

    let full_name = itertools::chain(&[root.as_str()], &name.segments).join(".");

    syn::Ident::new(full_name)
}

fn scope(name: token::Name, scope_level: u32, ast: ast::Ast, partials_resolver: &mut PartialsResolver)
    -> (syn::Ident, syn::Ident, quote::Tokens)
{
    let name = resolve(&name, scope_level);
    let scope_variable = syn::Ident::new(format!("_s{}", scope_level));
    let nested_generated = generate(ast, scope_level + 1, partials_resolver);

    (name, scope_variable, nested_generated)
}

pub fn generate(node: ast::Ast, scope_level: u32, partials_resolver: &mut PartialsResolver) -> quote::Tokens {
    use ast::Ast::*;
    match node {
        Sequence(seq) => {
            let items = seq.into_iter().map(|node| generate(node, scope_level, partials_resolver));
            quote! { #(#items)* }
        },
        Literal(text) => {
            quote! { f.write_str(#text)?; }
        },
        Interpolation(name) => {
            let name = resolve(&name, scope_level);
            quote! { _bart::DisplayHtmlSafe::safe_fmt(&#name, f)?; }
        },
        UnescapedInterpolation(name) => {
            let name = resolve(&name, scope_level);
            quote! { ::std::fmt::Display::fmt(&#name, f)?; }
        },
        Iteration { name, nested } => {
            let (name, scope_variable, nested) = scope(name, scope_level, *nested, partials_resolver);
            quote! {
                for ref #scope_variable in (&#name).into_iter() {
                    #nested
                }
            }
        },
        NegativeIteration { name, nested } => {
            let (name, scope_variable, nested) = scope(name, scope_level, *nested, partials_resolver);
            quote! {
                for ref #scope_variable in _bart::NegativeIterator::neg_iter(&#name) {
                    #nested
                }
            }
        },
        Conditional { name, nested } => {
            let (name, scope_variable, nested) = scope(name, scope_level, *nested, partials_resolver);
            quote! {
                if _bart::Conditional::val(&#name) {
                    let ref #scope_variable = #name;
                    #nested
                }
            }
        },
        NegativeConditional { name, nested } => {
            let (name, scope_variable, nested) = scope(name, scope_level, *nested, partials_resolver);
            quote! {
                if !_bart::Conditional::val(&#name) {
                    let ref #scope_variable = #name;
                    #nested
                }
            }
        },
        Scope { name, nested } => {
            let (name, scope_variable, nested) = scope(name, scope_level, *nested, partials_resolver);
            quote! {
                {
                    let ref #scope_variable = #name;
                    #nested
                }
            }
        },
        PartialInclude { partial_name, root } => {
            let root = resolve(&root, scope_level);
            let nested = partials_resolver.generate_partial(partial_name);
            quote! {
                {
                    let ref _s0 = #root;
                    #nested
                }
            }
        },
    }
}
