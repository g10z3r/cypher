use proc_macro2::TokenStream;

use crate::core::{ast, context::Context};

pub fn expand_derive_cypque(input: &mut syn::DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    let ctx = Context::new();
    let cont = match ast::Container::from_ast(&ctx, input) {
        Some(cont) => cont,
        None => return Err(ctx.check().unwrap_err()),
    };
    ctx.check()?;

    let props = collect_props(&cont)?;
    let labels = collect_labels(&cont)?;

    let node_query_name = &cont.attrs.name.settable;
    let node_ident_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let output = quote!(
        use cypher::query::{QueryTrait, Query};
        use cypher::entity::{Entity, Props, PropType, EntityTrait};

        impl #impl_generics EntityTrait for #node_ident_name #ty_generics #where_clause{
            fn into_entity(&self, nv: &str)-> Entity {
                use std::sync::Arc;

                let mut mp = Props::new();
                #(mp.insert(#props);)*

                let mut lb: Vec<Box<dyn Display>> = Vec::new();
                #(lb.push(#labels);)*

                Entity::node(
                    nv.clone().to_string(), 
                    (&#node_query_name), 
                    if mp.len() > 0 { Some(mp) } else { None }, 
                    if lb.len() > 0 { Some(lb) } else { None },
                )                
            }
        }
    );

    Ok(output)
}

/// Получение всех полей которые указаны как `label` узла.
fn collect_labels(cont: &ast::Container) -> Result<Vec<TokenStream>, Vec<syn::Error>> {
    let output = cont.data
        .1
        .iter()
        .filter(|field| !field.attrs.skip && field.attrs.label)
        .map(|field| {
            let org_name = field.original.ident.as_ref().unwrap();
            quote!(Box::new(self.#org_name.clone()))
        })
        .collect::<Vec<_>>();

    Ok(output)
}

/// Получение всех полей которые не помечены меткой `label`. 
/// Все собранные поля и их названия будут использоваться как параметры узла
/// при формировании запроса.
fn collect_props(cont: &ast::Container) -> Result<Vec<TokenStream>, Vec<syn::Error>> {
    let output = cont.data
        .1
        .iter()
        .filter(|field| !field.attrs.skip && !field.attrs.label)
        .map(|field| {
            // Нативное имя поля в родительской структуре
            let org_name = field.original.ident.as_ref().unwrap();
            // Имя параметра которое должно быть использовано при формировании запроса
            let set_name = field.attrs.name.settable.as_str();
            // Нативный тип поля в родительской структуре
            let mut _type = &field.original.ty;

            let prop_value =
                // Проверка, если тип поля опциональным
                if let Some(type_in_option) = ty_inner_type("Option", &field.original.ty) {
                    _type = type_in_option;                    
                    
                    // Проверка, если тип поля является массивом
                    if let Some(i_ty) = ty_inner_type("Vec", _type) {
                        let defval = match &field.attrs.default {
                            crate::core::attr::Default::None => quote!(None),
                            crate::core::attr::Default::Default => quote!(None),
                            crate::core::attr::Default::Value(value) => quote!(
                                Some(#value)
                            ),
                        };  
                    
                        quote!(
                            // Если значение сущенствует преoбразовываю массив в PropType::Array массив
                            if self.#org_name.is_some() {                                   
                                PropType::arr(
                                    stringify!(#i_ty), 
                                    self.#org_name.clone().unwrap(),
                                )                                
                            } else {
                                // Если будет указано дефолтное значение, тип преобразуется в PropType::StrArr
                                // и при формировании запроса результат будет идентичен с типом PropType::Array.
                                // Если дефолтного значения нет, при формировании запроса 
                                // будет использоваться тип PropType::Null.
                                PropType::str_arr(#defval)                         
                            }
                        )
                    } else {
                        // Если тип Option<T> равен Option::None и существует
                        // атрибут поля #[cypher(default = "...")]
                        let defval = match &field.attrs.default {
                            crate::core::attr::Default::None => quote!(None),
                            crate::core::attr::Default::Default => quote!(
                                Some(Box::new(#_type::default()))
                            ),
                            crate::core::attr::Default::Value(value) => quote!(
                                Some(Box::new(#value))
                            ),
                        };

                        quote!(
                            if self.#org_name.is_some() {                                   
                                PropType::from_type(
                                    stringify!(#_type), 
                                    Some(Box::new(self.#org_name.clone().unwrap())),
                                )                                
                            } else {
                                // Если было определено дефолтное значение, оно будет преобразовано
                                // в соответствующий тип PropType, иначе будет использоваться тип PropType::Null
                                PropType::from_type(stringify!(#_type), #defval)                            
                            }
                        )
                    }
                } else {
                    // Проверка, если тип поля является массивом
                    if let Some(i_ty) = ty_inner_type("Vec", _type) {
                        quote!(                            
                            PropType::arr(stringify!(#i_ty), self.#org_name.clone())
                        )
                    } else {
                        // Страндартное предоразование значения в один из типов PropType
                        quote!(                    
                            PropType::from_type(
                                stringify!(#_type), 
                                Some(Box::new(self.#org_name.clone())),
                            )
                        )
                    }
                };

            // Формирования финальной последовательность key, value для использования
            // в insert функции при сохранения параметров узла.
            quote! {
                #set_name.to_string(),
                #prop_value
            }
        })
        .collect::<Vec<_>>();

        Ok(output)
}

/// Определение родителя дженерик типа
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
