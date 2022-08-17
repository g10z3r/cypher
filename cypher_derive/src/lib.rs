mod core;

#[macro_use]
mod fragment;

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use syn::DeriveInput;

use crate::core::{ast, context::Context};

#[proc_macro_derive(Cypherize, attributes(cypher))]
pub fn derive_cypherize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    impl_cypherize(&input).into()
}

fn impl_cypherize(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let ctx = Context::new();
    let cont = match ast::Container::from_ast(&ctx, ast) {
        Some(cont) => cont,
        None => todo!(),
    };

    let variants = cont
        .data
        .1
        .into_iter()
        .map(|field| {
            let org_name = field.original.ident.as_ref().unwrap();
            let ser_name = field.attrs.name.serialize.as_str();
            let mut _type = &field.original.ty;

            let prop_value =
                if let Some(type_in_option) = ty_inner_type("Option", &field.original.ty) {
                    _type = type_in_option;

                    quote!(
                        if self.#org_name.is_some() {
                            Some(Box::new(self.#org_name.clone().unwrap()))
                        } else {
                            None
                        }
                    )
                } else {
                    quote!(Some(Box::new(self.#org_name.clone())))
                };

            quote! {
                #ser_name.to_string(),
                PropType::from_type(
                    stringify!(#_type),
                    #prop_value

                )
            }
        })
        .collect::<Vec<_>>();

    let name = &ast.ident;

    let output = quote!(
        use cypher::CypherTrait;
        use cypher::query::{QueryTrait, Query};
        use cypher::node::{Node, Props, PropType};




        impl CypherTrait for #name {
            fn cypher(&self) -> Box<dyn QueryTrait> {
                use std::sync::Arc;

                let mut mp = Props::new();
                #(
                    mp.insert(#variants);
                )*

                let node = Node::new(mp, vec![String::from(stringify!(#name))]);
                let q = Query::default(Arc::new(node));

                Box::new(q)
            }
        }



        // impl QueryTrait for Query<#name> {
        //     fn create(&mut self) -> Box<dyn WriteTrait> {
        //         let mut props = String::new();
        //         #(
        //             props.push_str(
        //                 #variants
        //             );
        //         )*
        //         props.pop();

        //         if props.len() > 0 {
        //             self.state = format!("CREATE (n:{} {{ {} }})", stringify!(#name), props);
        //         } else {
        //             self.state = format!("CREATE (n:{})", stringify!(#name));
        //         }

        //         Box::new(self.clone())
        //     }

        //     fn delete(&mut self, detach: bool) -> Box<dyn WriteTrait> {
        //         if detach {
        //             self.push_to_state(&format!("MATCH ({}:{})\nDETACH DELETE {}",
        //                 self.nv(),
        //                 stringify!(#name),
        //                 self.nv()
        //             ));

        //             Box::new(self.clone())
        //         } else {
        //             self.push_to_state(&format!("MATCH ({}:{})\nDELETE {}",
        //                 self.nv(),
        //                 stringify!(#name),
        //                 self.nv()
        //             ));

        //             Box::new(self.clone())
        //         }
        //     }
        // }

        // impl WriteTrait for Query<#name> {

        //     fn r#return(&mut self) -> Box<dyn ReturnTrait> {
        //         self.push_to_state(&format!("\nRETURN {}", self.nv()));
        //         Box::new(self.clone())
        //     }

        //     fn return_as(&mut self, value: &str) -> Box<dyn ReturnTrait> {
        //         self.push_to_state(&format!("\nRETURN {} AS {}", self.nv(), value));
        //         Box::new(self.clone())
        //     }
        // }

        // impl ReturnTrait for Query<#name> {
        //     #finalize
        // }
    );

    output
}

fn ty_inner_type<'a>(wrapper: &str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != wrapper {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            let inner_ty = inner_ty.args.first().unwrap();
            if let syn::GenericArgument::Type(ref t) = inner_ty {
                return Some(t);
            }
        }
    }
    None
}
