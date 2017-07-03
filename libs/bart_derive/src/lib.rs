#[macro_use] extern crate quote;

extern crate proc_macro;
extern crate syn;

mod ast;
mod generator;
mod parser;
mod scanner;
mod token;

use ast::Ast;
use proc_macro::TokenStream;
use std::fs::File;
use std::io::prelude::*;

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

fn parse_str(input: &str) -> Result<Ast, parser::Error> {
    parser::parse(scanner::sequence(input).unwrap())
}

#[proc_macro_derive(BartDisplay, attributes(template, template_string, template_root))]
pub fn bart_display(input: TokenStream) -> TokenStream {
    use std::env;

    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();

    let user_crate_root = env::current_dir().expect("Unable to get current directory");

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

    let template_root = syn::Ident::new(find_attr(&ast.attrs, "template_root")
        .map(|x| scanner::segmented_name(&x).expect("Syntax error in template_root"))
        .map(|x| format!("self.{}", x.join(".")))
        .unwrap_or("self".to_owned()));

    let parsed = parse_str(&template).unwrap();
    let generated = generator::generate(parsed, 1);

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

                    let ref _s0 = #template_root;

                    #generated

                    Ok(())
                }
            }
        };
    };

    gen.parse().unwrap()
}
