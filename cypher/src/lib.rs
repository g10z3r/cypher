pub mod node;

use std::{collections::HashMap, fmt::format, sync::Arc};

#[cfg(feature = "derive")]
pub use cypher_derive::Cypherize;

use node::{Node, Props};

pub struct Execute(String);

pub trait ExecuteTrait: 'static {
    fn execute(&self) -> String;
}

impl ExecuteTrait for Execute {
    fn execute(&self) -> String {
        self.0.clone()
    }
}

pub struct ReturnQuery {
    nv: String,
    state: String,
    data: Arc<Node>,
}
pub trait ReturnTrait: 'static {
    fn r#return(&self) -> Box<dyn ExecuteTrait>;
}

pub struct Query {
    nv: String,
    state: String,
    data: Arc<Node>,
}

impl Query {
    pub fn new(nv: String, data: Arc<Node>) -> Self {
        Query {
            nv,
            state: String::new(),
            data,
        }
    }

    pub fn default(data: Arc<Node>) -> Self {
        Query::new(String::from("n"), data)
    }
}

pub trait QueryTrait: 'static {
    fn create(&self) -> Box<dyn ReturnTrait>;
}

impl QueryTrait for Query {
    fn create(&self) -> Box<dyn ReturnTrait> {
        let state = if self.data.props().len() > 0 {
            let mut props: String = self
                .data
                .props()
                .into_iter()
                .filter(|(_, v)| v.is_some())
                .map(|(k, v)| format!("{}: {},", k, v.to_prop()))
                .collect();
            props.pop();

            format!(
                "CREATE ({}:{} {{ {} }})",
                self.nv,
                self.data.get_label(0).unwrap(),
                props
            )
        } else {
            format!("CREATE ({}:{})", self.nv, self.data.get_label(0).unwrap())
        };

        let rq = ReturnQuery {
            nv: self.nv.clone(),
            state,
            data: self.data.clone(),
        };

        Box::new(rq)
    }
}

impl ReturnTrait for ReturnQuery {
    fn r#return(&self) -> Box<dyn ExecuteTrait> {
        Box::new(Execute(self.state.clone()))
    }
}

pub trait CypherTrait: Sized + 'static {
    fn cypher(&self) -> Box<dyn QueryTrait>;
}

struct Test {
    dd: i32,
}

impl CypherTrait for Test {
    fn cypher(&self) -> Box<dyn QueryTrait> {
        let mut mp: Props = HashMap::new();

        // mp.insert("".to_string(), Some(Box::new("".to_string())));

        let node = Node::new(mp, Vec::new());

        let q = Query::default(std::sync::Arc::new(node));

        Box::new(q)
    }
}

fn test<'a>() {
    let r = Test { dd: 32 };

    let y = r.cypher().create().r#return().execute();
}
