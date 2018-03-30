#![cfg(proc_macro)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(ZeroedMem)]
pub fn zeroed_mem(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let derive_input = syn::parse_derive_input(&s).unwrap();
    let gen = impl_zeroed_mem(&derive_input);
    gen.parse().unwrap()
}

fn impl_zeroed_mem(ipt: &syn::DeriveInput) -> quote::Tokens {
    let name = &ipt.ident;
    quote! {
        impl ZeroedMem for #name {}
    }
}

#[proc_macro_derive(Inspect)]
pub fn inspect(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let derive_input = syn::parse_derive_input(&s).unwrap();
    let gen = impl_inspect(&derive_input);
    gen.parse().unwrap()
}

fn impl_inspect(ipt: &syn::DeriveInput) -> quote::Tokens {
    let name = &ipt.ident;
    //let name = name.to_string() + "_foo";
    let varname = format!("FOO_{}", name);
    let varname = quote::Ident::from(varname);
    quote! {
        lazy_static! {
            static ref #varname : u32 = tyty_add(stringify!(#name));
        }
        /*
        lazy_static! {
            static ref foo = #varname;
        }
        */
    }
}

#[proc_macro_derive(Foo, attributes(tie, hint))]
pub fn foo(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let derive_input = syn::parse_derive_input(&s).unwrap();
    let gen = impl_foo(&derive_input);
    gen.parse().unwrap()
}
fn impl_foo(ipt: &syn::DeriveInput) -> quote::Tokens {
    let name = &ipt.ident;
    let attrs = impl_foo_attrs(&ipt.attrs);
    let content = format!("{:?}", ipt);
    let fields = impl_foo_fields(ipt);
    quote! {
        impl #name {
            fn foo() {
                println!("Attrs : {} ", #attrs);
                println!("Fields : {} ", #fields);
                println!("Content : {:?}", #content);
            }
        }
    }
}
fn impl_foo_attrs(attrs : &Vec<syn::Attribute>) -> String {
    let mut s = String::new() + "\n";
    for a in attrs {
        match &a.value {
            &syn::MetaItem::List(ref ident, ref items) => {
                match format!("{}", ident).as_str() {
                    "tie" | "hint" => {}
                    _ => continue
                }
                s += format!("{}(", ident).as_str();
                for it in items {
                    match it {
                        &syn::NestedMetaItem::MetaItem(ref mitem) => {
                            match mitem {
                                &syn::MetaItem::Word(ref ident) => {
                                    s += format!("{}, ", ident).as_str();
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                s += ")\n";
            }
            _ => {}
        }
    }
    s //+ format!("{:?}", attrs).as_str()
}
fn impl_foo_fields(derive_input : &syn::DeriveInput) -> String {
    let mut s = String::new() + "\n";
    match &derive_input.body {
        &syn::Body::Enum(_) => (),
        &syn::Body::Struct(ref st) => {
            match st {
                &syn::VariantData::Struct(ref fields) => {
                    for f in fields {
                        s += format!("    id:{} ", f.ident.clone().unwrap()).as_str();
                        match f.ty.clone() {
                            syn::Ty::Path(_,path) => {
                                s += format!("ty:{} ", path.segments[0].ident).as_str();
                            }
                            _ => (),
                        }
                        s += "\n";
                    }
                },
                &syn::VariantData::Tuple(_) => (),
                &syn::VariantData::Unit => (),
            }
        }
    }
    s
}

/*
#[proc_macro_derive(Foo)]
pub fn foo(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = impl_foo(&ast);
    gen.parse().unwrap()
}
fn impl_foo(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl #name {
            fn foo() {
                println!("Hello, World! My name is {}", stringify!(#name));
            }
        }
    }
}
*/
