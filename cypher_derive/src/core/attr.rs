use super::context::Context;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;
use syn::Ident;
use syn::Meta::{List, NameValue, Path};
use syn::NestedMeta::{Lit, Meta};

use super::symbol::*;

pub struct Attr<'a, T> {
    ctx: &'a Context,
    name: Symbol,
    tokens: TokenStream,
    value: Option<T>,
}

impl<'a, T> Attr<'a, T> {
    fn none(ctx: &'a Context, name: Symbol) -> Self {
        Attr {
            ctx,
            name,
            tokens: TokenStream::new(),
            value: None,
        }
    }

    fn set<A: ToTokens>(&mut self, obj: A, value: T) {
        let tokens = obj.into_token_stream();

        if self.value.is_some() {
            self.ctx.error_spanned_by(
                tokens,
                format!("duplicate cypher attribute `{}`", self.name),
            );
        } else {
            self.tokens = tokens;
            self.value = Some(value);
        }
    }

    fn set_opt<A: ToTokens>(&mut self, obj: A, value: Option<T>) {
        if let Some(value) = value {
            self.set(obj, value);
        }
    }

    fn set_if_none(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
        }
    }

    fn get(self) -> Option<T> {
        self.value
    }
}

struct BoolAttr<'c>(Attr<'c, ()>);

impl<'c> BoolAttr<'c> {
    fn none(cx: &'c Context, name: Symbol) -> Self {
        BoolAttr(Attr::none(cx, name))
    }

    fn set_true<A: ToTokens>(&mut self, obj: A) {
        self.0.set(obj, ());
    }

    fn get(&self) -> bool {
        self.0.value.is_some()
    }
}

pub struct Field {
    pub name: Name,
    pub skip: bool,
}

impl Field {
    pub fn from_ast(
        ctx: &Context,
        index: usize,
        field: &syn::Field,
        attrs: Option<&Variant>,
    ) -> Self {
        let mut set_name = Attr::none(ctx, RENAME);
        let mut get_name = Attr::none(ctx, RENAME);

        let mut skip = BoolAttr::none(ctx, SKIP);

        let ident = match &field.ident {
            Some(ident) => unraw(ident),
            None => index.to_string(),
        };

        for meta_item in field
            .attrs
            .iter()
            .flat_map(|attr| get_serde_meta_inputs(ctx, attr))
            .flatten()
        {
            match &meta_item {
                // Parse `#[cypher(rename = "name")]`
                Meta(NameValue(m)) if m.path == RENAME => {
                    if let Ok(s) = get_lit_str(ctx, RENAME, &m.lit) {
                        set_name.set(&m.path, s.value());
                    }
                }

                // Parse `#[cypher(skip)]`
                Meta(Path(word)) if word == SKIP => skip.set_true(word),

                Meta(meta_item) => {
                    let path = meta_item
                        .path()
                        .into_token_stream()
                        .to_string()
                        .replace(' ', "");
                    ctx.error_spanned_by(
                        meta_item.path(),
                        format!("unknown cypher field attribute `{}`", path),
                    );
                }

                Lit(lit) => {
                    ctx.error_spanned_by(lit, "unexpected literal in cypher field attribute");
                }
            }
        }

        Field {
            name: Name::from_attrs(ident, set_name, get_name),
            skip: skip.get(),
        }
    }
}

pub struct Name {
    pub serialize: String,
    pub serialize_renamed: bool,
    pub deserialize: String,
    pub deserialize_renamed: bool,
}

impl Name {
    fn from_attrs(source_name: String, ser_name: Attr<String>, de_name: Attr<String>) -> Name {
        let ser_name = ser_name.get();
        let ser_renamed = ser_name.is_some();
        let de_name = de_name.get();
        let de_renamed = de_name.is_some();

        Name {
            serialize: ser_name.unwrap_or_else(|| source_name.clone()),
            serialize_renamed: ser_renamed,
            deserialize: de_name.unwrap_or(source_name),
            deserialize_renamed: de_renamed,
        }
    }
}

/// Контейнер исходной информации об атрибутах структуры
pub struct Container {
    pub name: Name,
}

impl Container {
    /// Извлечение атрибутов элемента
    pub fn from_ast(ctx: &Context, input: &DeriveInput) -> Self {
        let mut set_name = Attr::none(ctx, RENAME);
        let mut get_name = Attr::none(ctx, RENAME);

        for meta_input in input
            .attrs
            .iter()
            .flat_map(|attr| get_serde_meta_inputs(ctx, attr))
            .flatten()
        {
            match &meta_input {
                Meta(NameValue(m)) if m.path == RENAME => {
                    if let Ok(s) = get_lit_str(ctx, RENAME, &m.lit) {
                        set_name.set(&m.path, s.value());
                    }
                }

                Meta(meta_item) => {
                    let path = meta_item
                        .path()
                        .into_token_stream()
                        .to_string()
                        .replace(' ', "");
                    ctx.error_spanned_by(
                        meta_item.path(),
                        format!("unknown cypher container attribute `{}`", path),
                    );
                }

                Lit(lit) => {
                    ctx.error_spanned_by(lit, "unexpected literal in serde container attribute");
                }
            }
        }

        Container {
            name: Name::from_attrs(unraw(&input.ident), set_name, get_name),
        }
    }
}

pub struct Variant {
    name: Name,
}

impl<'a> Variant {
    pub fn from_ast(ctx: &'a Context, variant: &'a syn::Variant) -> Self {
        let mut get_name = Attr::none(ctx, RENAME);
        let mut set_name = Attr::none(ctx, RENAME);

        for meta_item in variant
            .attrs
            .iter()
            .flat_map(|attr| get_serde_meta_inputs(ctx, attr))
            .flatten()
        {
            match &meta_item {
                Meta(NameValue(m)) if m.path == RENAME => {
                    if let Ok(s) = get_lit_str(ctx, RENAME, &m.lit) {
                        set_name.set(&m.path, s.value());
                    }
                }

                Meta(meta_item) => {
                    let path = meta_item
                        .path()
                        .into_token_stream()
                        .to_string()
                        .replace(' ', "");
                    ctx.error_spanned_by(
                        meta_item.path(),
                        format!("unknown cypher variant attribute `{}`", path),
                    );
                }

                Lit(lit) => {
                    ctx.error_spanned_by(lit, "unexpected literal in serde variant attribute");
                }
            }
        }

        Variant {
            name: Name::from_attrs(unraw(&variant.ident), set_name, get_name),
        }
    }
}

fn unraw(ident: &Ident) -> String {
    ident.to_string().trim_start_matches("r#").to_owned()
}

fn get_lit_str<'a>(
    ctx: &Context,
    attr_name: Symbol,
    lit: &'a syn::Lit,
) -> Result<&'a syn::LitStr, ()> {
    if let syn::Lit::Str(lit) = lit {
        Ok(lit)
    } else {
        ctx.error_spanned_by(
            lit,
            format!(
                "expected cypher {} attribute to be a string: `{} = \"...\"`",
                attr_name, attr_name
            ),
        );

        Err(())
    }
}

fn get_serde_meta_inputs(ctx: &Context, attr: &syn::Attribute) -> Result<Vec<syn::NestedMeta>, ()> {
    match attr.parse_meta() {
        Ok(List(meta)) => Ok(meta.nested.into_iter().collect()),
        Ok(other) => {
            ctx.error_spanned_by(other, "expected #[cypher(...)]");
            Err(())
        }
        Err(err) => {
            ctx.syn_error(err);
            Err(())
        }
    }
}
