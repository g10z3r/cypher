use std::{collections::HashMap, fmt::Display};

pub enum PropType {
    Int(Box<dyn Display + 'static>),
    String(Box<dyn Display + 'static>),
    Bool(bool),
    Array(Vec<Self>),
    /// If a well-formed array already exists as a string.
    /// For example: `['Bob', 'Tom']`.
    ///
    /// When forming a request, the result will be identical
    /// to the `PropType::Array` type.
    StrArr(String),
    /// Neo4j BOLT type NULL
    Null,
}

impl PropType {
    /// Create properties type `Int`
    pub fn int<T>(value: T) -> PropType
    where
        T: std::fmt::Display + 'static,
    {
        PropType::Int(Box::new(value))
    }

    /// Create properties type `String`
    pub fn str<T>(value: T) -> PropType
    where
        T: std::fmt::Display + 'static,
    {
        PropType::String(Box::new(value))
    }

    /// Create properties type `Array`
    pub fn arr<T>(ty: &str, value: Vec<T>) -> PropType
    where
        T: std::fmt::Display + 'static,
    {
        PropType::Array(
            value
                .into_iter()
                .map(|item| PropType::from_type(ty, Some(Box::new(item))))
                .collect::<Vec<Self>>(),
        )
    }

    /// Create type PropType::Array if the array is already well-formed.
    /// For example: `['Bob', 'Tom']`.
    pub fn str_arr(value: Option<&str>) -> PropType {
        if let Some(value) = value {
            PropType::StrArr(value.to_string())
        } else {
            PropType::Null
        }
    }
}

impl PartialEq for PropType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(_), Self::Int(_)) => true,
            (Self::String(_), Self::String(_)) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl PropType {
    pub fn from_type(tt: &str, value: Option<Box<dyn Display + 'static>>) -> PropType {
        if let Some(value) = value {
            return match tt {
                "String" => PropType::String(value),
                "i128" | "i64" | "i32" | "i16" | "i8" => PropType::Int(value),
                "u128" | "u64" | "u32" | "u16" | "u8" => PropType::Int(value),
                "usize" | "isize" => PropType::Int(value),
                "bool" => PropType::Bool(if value.to_string() == "true" {
                    true
                } else {
                    false
                }),

                _ => PropType::String(value),
            };
        };

        return PropType::Null;
    }

    pub fn to_prop(&self) -> String {
        match self {
            PropType::Int(value) => value.as_ref().to_string(),
            PropType::String(value) => format!("'{}'", value.as_ref()),
            PropType::Bool(value) => value.to_string(),
            PropType::Array(value) => {
                let mut body = value
                    .iter()
                    .map(|item| format!("{},", PropType::to_prop(item)))
                    .collect::<String>();
                body.pop();

                format!("[{}]", body)
            }
            PropType::StrArr(value) => value.to_string(),
            PropType::Null => String::from("NULL"),
        }
    }
}

pub type Props = HashMap<String, PropType>;
pub type Label = Box<dyn Display>;

pub trait EntityTrait: 'static + Sized {
    fn entity(&self, nv: &str) -> Entity;
}

pub enum Entity<'a> {
    Node {
        nv: String,
        node_name: &'a str,
        props: Props,
        labels: Vec<Label>,
    },

    Relation,
}

impl<'a> Entity<'a> {
    pub fn node(nv: String, node_name: &'a str, props: Props, labels: Vec<Label>) -> Entity<'a> {
        Entity::Node {
            nv,
            node_name,
            props,
            labels,
        }
    }
}
