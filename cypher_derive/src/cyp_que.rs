use proc_macro2::TokenStream;

use crate::core::{ast, context::Context};

pub fn expand_derive_cypue(input: &mut syn::DeriveInput) -> TokenStream {
    let ctx = Context::new();
    let cont = match ast::Container::from_ast(&ctx, input) {
        Some(cont) => cont,
        None => todo!(),
    };

    let variants = cont
        .data
        .1
        .into_iter()
        .filter(|field| !field.attrs.skip)
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

    let node_ident_name = &input.ident;
    let node_query_name = cont.attrs.name.serialize;

    let output = quote!(
        use cypher::CypherTrait;
        use cypher::query::{QueryTrait, Query};
        use cypher::node::{Node, Props, PropType};

        impl CypherTrait for #node_ident_name {
            fn cypher(&self) -> Box<dyn QueryTrait> {
                use std::sync::Arc;

                let mut mp = Props::new();
                #(
                    mp.insert(#variants);
                )*

                let node = Node::new(mp, vec![String::from(#node_query_name)]);
                let q = Query::default(Arc::new(node));

                Box::new(q)
            }
        }
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
