#[macro_use] extern crate nom;
#[macro_use] extern crate quick_error;
#[macro_use] extern crate quote;

extern crate proc_macro;
extern crate syn;

mod ast;
mod parsbart;

use proc_macro::TokenStream;

use ast::Ast;
use std::fs::File;
use std::io::prelude::*;

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
            quote! { _bart::DisplayHtmlSafe::safe_fmt(&#name, f)?; }
        },
        UnescapedInterpolation(name) => {
            let name = syn::Ident::new(name.resolve(scope_level));
            quote! { ::std::fmt::Display::fmt(&#name, f)?; }
        },
        Iteration { name, nested } => {
            let name = syn::Ident::new(name.resolve(scope_level));
            let scope_variable = syn::Ident::new(format!("_s{}", scope_level));
            let nested_generated = generate(*nested, scope_level + 1);
            quote! {
                for ref #scope_variable in (&#name).into_iter() {
                    #nested_generated
                }
            }
        },
        Conditional { name, nested } => {
            let name = syn::Ident::new(name.resolve(scope_level));
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
            let name = syn::Ident::new(name.resolve(scope_level));
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

fn find_attr<'a>(attrs: &'a Vec<syn::Attribute>, name: &str) -> Option<&'a str> {
    attrs.iter()
        .find(|&x| x.name() == name)
        .and_then(|ref attr| match &attr.value {
            &syn::MetaItem::NameValue(_, syn::Lit::Str(ref template, _)) => Some(template),
            _ => None
        })
        .map(|x| x.as_ref())
}

fn buf_file(filename: &str) -> String {
    let mut f = File::open(filename)
        .expect("Unable to open file for reading");
    let mut buf = String::new();
    f.read_to_string(&mut buf)
        .expect("Unable to read file");

    buf
}

#[proc_macro_derive(BartDisplay, attributes(template, template_string))]
pub fn bart_display(input: TokenStream) -> TokenStream {
    use std::env;
    use std::path::PathBuf;

    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();

    let user_crate_root = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR must be set to the root path of the crate")
    );

    let mut dependencies = Vec::<String>::new();

    let filename_opt = find_attr(&ast.attrs, "template");

    let template =
        filename_opt.map(buf_file)
            .or_else(||
                find_attr(&ast.attrs, "template_string").map(|x| x.to_owned())
            )
        .expect("#[derive(BartDisplay)] requires #[template = \"(filename)\"] or  #[template_string = \"...\"]");

    if let Some(filename) = filename_opt {
        dependencies.push(user_crate_root.join(filename).to_str().unwrap().to_owned());
    }

    let parsed = parsbart::parse_str(&template).unwrap();
    let generated = generate(parsed, 1);

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let dummy_const = syn::Ident::new(format!("_IMPL_BART_DISPLAY_FOR_{}", &name));

    let gen = quote! {
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const #dummy_const: () = {
            extern crate bart as _bart;

            #[automatically_derived]
            impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    #(
                        let _ = include_bytes!(#dependencies);
                    )*

                    let ref _s0 = self;

                    #generated

                    Ok(())
                }
            }
        };
    };

    gen.parse().unwrap()
}
