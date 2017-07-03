#[macro_use] extern crate quote;

extern crate itertools;
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
use std::path::{Path, PathBuf};

fn find_attr<'a>(attrs: &'a Vec<syn::Attribute>, name: &str) -> Option<&'a str> {
    attrs.iter()
        .find(|&x| x.name() == name)
        .and_then(|ref attr| match &attr.value {
            &syn::MetaItem::NameValue(_, syn::Lit::Str(ref template, _)) => Some(template),
            _ => None
        })
        .map(|x| x.as_ref())
}

fn buf_file<P: AsRef<Path>>(filename: P) -> String {
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

struct InlinePartialsResolver;
impl generator::PartialsResolver for InlinePartialsResolver {
    fn generate_partial(&mut self, _partial_name: &str) -> quote::Tokens {
        panic!("Partials are unavailable when using template_string");
    }
}

struct FilesystemPartialsResolver<'a> {
    base_dir: PathBuf,
    dependencies: &'a mut Vec<String>,
}

impl<'a> FilesystemPartialsResolver<'a> {
    fn new<T: Into<PathBuf>>(base_dir: T, dependencies: &mut Vec<String>) -> FilesystemPartialsResolver {
        FilesystemPartialsResolver {
            base_dir: base_dir.into(),
            dependencies,
        }
    }
}

impl<'a> generator::PartialsResolver for FilesystemPartialsResolver<'a> {
    fn generate_partial(&mut self, partial_name: &str) -> quote::Tokens {
        let abs_path = self.base_dir.join(partial_name);
        self.dependencies.push(abs_path.to_str().unwrap().to_owned());
        let template = buf_file(&abs_path);
        let parsed = parse_str(&template).unwrap();
        let nested_resolver = &mut FilesystemPartialsResolver::new(abs_path.parent().unwrap(), self.dependencies);
        generator::generate(parsed, 1, nested_resolver)
    }
}

#[proc_macro_derive(BartDisplay, attributes(template, template_string, template_root))]
pub fn bart_display(input: TokenStream) -> TokenStream {
    use std::env;

    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();

    let user_crate_root = env::current_dir().expect("Unable to get current directory");

    let mut dependencies = Vec::<String>::new();

    let generated = {
        let (template, mut partials_resolver): (_, Box<generator::PartialsResolver>) =
            match find_attr(&ast.attrs, "template") {
                Some(filename) => {
                    let abs_filename = user_crate_root.join(filename);
                    dependencies.push(abs_filename.to_str().unwrap().to_owned());
                    let resolver = FilesystemPartialsResolver::new(abs_filename.parent().unwrap(), &mut dependencies);
                    (buf_file(filename), Box::new(resolver))
                },
                None => {
                    let template = find_attr(&ast.attrs, "template_string")
                        .map(|x| x.to_owned())
                        .expect("#[derive(BartDisplay)] requires #[template = \"(filename)\"] \
                            or  #[template_string = \"...\"]");
                    (template, Box::new(InlinePartialsResolver))
                }
            };

        let parsed = parse_str(&template).unwrap();
        generator::generate(parsed, 1, &mut *partials_resolver)
    };

    let template_root = syn::Ident::new(find_attr(&ast.attrs, "template_root")
        .map(|x| scanner::segmented_name(&x).expect("Syntax error in template_root"))
        .map(|x| format!("self.{}", x.join(".")))
        .unwrap_or("self".to_owned()));

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
