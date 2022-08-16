use std::{collections::HashMap, fmt::Display};

pub enum PropType {
    Int(Box<dyn Display + 'static>),
    String(Box<dyn Display + 'static>),
    Null,
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

                _ => PropType::String(value),
            };
        };

        return PropType::Null;
    }

    pub fn to_prop(&self) -> String {
        match self {
            PropType::Int(value) => value.as_ref().to_string(),
            PropType::String(value) => format!("'{}'", value.as_ref()),
            PropType::Null => String::from("NULL"),
        }
    }
}

pub type Props = HashMap<String, PropType>;
pub type Label = String;

pub struct Node {
    labels: Vec<Label>,
    props: Props,
}

impl Node {
    pub fn new(props: Props, labels: Vec<Label>) -> Self {
        Node { props, labels }
    }

    pub fn props(&self) -> &Props {
        &self.props
    }

    pub fn add_label(&mut self, label: Label) {
        self.labels.push(label)
    }

    pub fn get_label(&self, index: usize) -> Option<&Label> {
        self.labels.get(index)
    }
}