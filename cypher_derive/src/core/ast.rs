use syn::punctuated::Punctuated;

use super::{attr, context::Context};

/// Исходная структура данных, аннотированная #[derive(Cypherize)]
pub struct Container<'a> {
    pub ident: syn::Ident,
    pub attrs: attr::Container,
    pub data: (Style, Vec<Field<'a>>),
    pub generics: &'a syn::Generics,
    pub original: &'a syn::DeriveInput,
}

pub enum Style {
    Struct,
    Newtype,
}

pub struct Field<'a> {
    pub member: syn::Member,
    pub attrs: attr::Field,
    pub ty: &'a syn::Type,
    pub original: &'a syn::Field,
}

impl<'a> Container<'a> {
    pub fn from_ast(ctx: &'a Context, input: &'a syn::DeriveInput) -> Option<Container<'a>> {
        let attrs = attr::Container::from_ast(ctx, input);

        let data = match &input.data {
            syn::Data::Struct(data) => struct_from_ast(ctx, &data.fields, None),
            syn::Data::Enum(_) => todo!(),
            syn::Data::Union(_) => todo!(),
        };

        let item = Container {
            ident: input.ident.clone(),
            attrs,
            data,
            generics: &input.generics,
            original: input,
        };

        Some(item)
    }
}

fn struct_from_ast<'a>(
    ctx: &'a Context,
    fields: &'a syn::Fields,
    attrs: Option<&attr::Variant>,
) -> (Style, Vec<Field<'a>>) {
    match fields {
        syn::Fields::Named(fields) => (Style::Struct, fields_from_ast(ctx, &fields.named, attrs)),
        syn::Fields::Unnamed(_) => todo!(),
        syn::Fields::Unit => todo!(),
    }
}

fn fields_from_ast<'a>(
    ctx: &'a Context,
    fields: &'a Punctuated<syn::Field, Token![,]>,
    attrs: Option<&attr::Variant>,
) -> Vec<Field<'a>> {
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| Field {
            member: match &field.ident {
                Some(ident) => syn::Member::Named(ident.clone()),
                None => syn::Member::Unnamed(i.into()),
            },
            attrs: attr::Field::from_ast(ctx, i, field, attrs),
            ty: &field.ty,
            original: field,
        })
        .collect()
}
