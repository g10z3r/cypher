mod core;

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use std::sync::Arc;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort, proc_macro_error};
use proc_macro_roids::{DeriveInputStructExt, FieldExt, IdentExt};
use quote::ToTokens;
use syn::{spanned::Spanned, DeriveInput, Ident};

use crate::core::{ast, context::Context};

#[proc_macro_derive(Cypherize, attributes(cypher))]
pub fn derive_cypherize(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    impl_cypherize(&input).into()
}

fn impl_cypherize(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let ctx = Context::new();
    let cont = match ast::Container::from_ast(&ctx, ast) {
        Some(cont) => cont,
        None => todo!(),
    };

    // let ff = cont.data.1;

    // let fields = collect_fields(&ast);

    // let relevant_fields = fields
    //     .iter()
    //     .filter(|field| !field.is_phantom_data())
    //     .filter(|field| !field.contains_tag(&parse_quote!(super_derive), &parse_quote!(skip)));

    let variants = cont
        .data
        .1
        .into_iter()
        .map(|field| {
            let org_name = field.original.ident.as_ref().unwrap();
            let ser_name = field.attrs.name.serialize.as_str();

            quote! {
                &format!("{}: '{}',", #ser_name, self.source.#org_name)
            }
        })
        .collect::<Vec<_>>();

    let name = &ast.ident;

    let output = quote!(
        use cypher::{QueryTrait, CreateTrait, ReturnTrait};

        impl QueryTrait for Query<#name> {
            fn create(&mut self) -> Box<dyn CreateTrait> {
                let mut props = String::new();
                #(
                    props.push_str(
                        #variants
                    );
                )*
                props.pop();

                if props.len() > 0 {
                    self.state = format!("CREATE (n:{} {{ {} }})", stringify!(#name), props);
                } else {
                    self.state = format!("CREATE (n:{})", stringify!(#name));
                }


                // self.source

                Box::new(self.clone())
            }
        }

        impl CreateTrait for Query<#name> {
            fn r#return(&mut self) -> Box<dyn ReturnTrait> {
                Box::new(self.clone())
            }
        }

        impl ReturnTrait for Query<#name> {
            fn finalize(&self) -> String {
                self.state.clone()
            }
        }

        // impl #name {
        //     fn test(&self) -> String {
        //         let mut props = String::new();
        //         #(
        //             props.push_str(
        //                 #variants
        //             );
        //         )*
        //         props.pop();

        //         if props.len() > 0 {
        //             format!("CREATE (n:{} {{ {} }})", stringify!(#name), props)
        //         } else {
        //             format!("CREATE (n:{})", stringify!(#name))
        //         }
        //     }
        // }
    );

    output
}

fn collect_fields(ast: &syn::DeriveInput) -> Vec<syn::Field> {
    match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            if fields.iter().any(|field| field.ident.is_none()) {
                abort!(
                    fields.span(),
                    "struct has unnamed fields";
                    help = "#[derive(Cypherize)] can only be used on structs with named fields";
                );
            }
            fields.iter().cloned().collect::<Vec<_>>()
        }
        _ => abort!(
            ast.span(),
            "#[derive(Cypherize)] can only be used with structs"
        ),
    }
}
