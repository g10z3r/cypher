use std::{collections::HashMap, fmt::Display};

pub enum PropType {
    Int(Option<Box<dyn Display + 'static>>),
    String(Option<Box<dyn Display + 'static>>),
}

impl PropType {
    pub fn from_type(tt: &str, value: Option<Box<dyn Display + 'static>>) -> PropType {
        match tt {
            "String" => PropType::String(value),
            "i32" => PropType::Int(value),

            _ => PropType::String(value),
        }
    }

    pub fn to_prop(&self) -> String {
        match self {
            PropType::Int(value) => value.as_ref().unwrap().to_string(),
            PropType::String(value) => format!("'{}'", value.as_ref().unwrap()),
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            PropType::Int(value) => value.is_some(),
            PropType::String(value) => value.is_some(),
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
        let r: isize = 1;

        let d: isize = r.into();

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
