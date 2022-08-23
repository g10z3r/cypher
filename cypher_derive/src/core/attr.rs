use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{self, Parse, ParseStream};
use syn::DeriveInput;
use syn::Ident;
use syn::Meta::{List, NameValue, Path};
use syn::NestedMeta::{Lit, Meta};

use super::context::Context;
use super::respan::*;
use super::symbol::*;

pub enum Default {
    None,
    Default,
    Value(String),
}

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
    pub label: bool,
    pub default: Default,
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
        let mut label = BoolAttr::none(ctx, LABEL);
        let mut default = Attr::none(ctx, DEFAULT);

        let ident = match &field.ident {
            Some(ident) => unraw(ident),
            None => index.to_string(),
        };

        for meta_item in field
            .attrs
            .iter()
            .flat_map(|attr| get_cypher_meta_inputs(ctx, attr))
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

                // Parse `#[cypher(label)]`
                Meta(Path(word)) if word == LABEL => label.set_true(word),

                // Parse `#[cypher(default)]`
                Meta(Path(word)) if word == DEFAULT => default.set(word, Default::Default),

                // Parse `#[cypher(default = "...")]`
                Meta(NameValue(m)) if m.path == DEFAULT => {
                    if let Ok(s) = get_lit_str(ctx, RENAME, &m.lit) {
                        default.set(&m.path, Default::Value(s.value()));
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
            label: label.get(),
            default: default.get().unwrap_or(Default::None),
        }
    }
}

pub struct Name {
    pub settable: String,
    pub settable_renamed: bool,
    pub gettable: String,
    pub gettable_renamed: bool,
}

impl Name {
    fn from_attrs(source_name: String, set_name: Attr<String>, get_name: Attr<String>) -> Name {
        let set_name = set_name.get();
        let set_renamed = set_name.is_some();
        let get_name = get_name.get();
        let get_renamed = get_name.is_some();

        Name {
            settable: set_name.unwrap_or_else(|| source_name.clone()),
            settable_renamed: set_renamed,
            gettable: get_name.unwrap_or(source_name),
            gettable_renamed: get_renamed,
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
            .flat_map(|attr| get_cypher_meta_inputs(ctx, attr))
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
                    ctx.error_spanned_by(lit, "unexpected literal in cypher container attribute");
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
            .flat_map(|attr| get_cypher_meta_inputs(ctx, attr))
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
                    ctx.error_spanned_by(lit, "unexpected literal in cypher variant attribute");
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

fn get_cypher_meta_inputs(
    ctx: &Context,
    attr: &syn::Attribute,
) -> Result<Vec<syn::NestedMeta>, ()> {
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

fn parse_lit_into_expr_path(
    cx: &Context,
    attr_name: Symbol,
    lit: &syn::Lit,
) -> Result<syn::ExprPath, ()> {
    let string = get_lit_str(cx, attr_name, lit)?;
    parse_lit_str(string).map_err(|_| {
        cx.error_spanned_by(lit, format!("failed to parse path: {:?}", string.value()));
    })
}

fn parse_lit_str<T>(s: &syn::LitStr) -> parse::Result<T>
where
    T: Parse,
{
    let tokens = spanned_tokens(s)?;
    syn::parse2(tokens)
}

fn spanned_tokens(s: &syn::LitStr) -> parse::Result<TokenStream> {
    let stream = syn::parse_str(&s.value())?;
    Ok(respan(stream, s.span()))
}
