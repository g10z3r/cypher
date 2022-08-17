pub mod execute;
pub mod match_query;
pub mod return_query;

use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use crate::node::{Node, PropType};
use crate::query::match_query::{MatchQuery, MatchTrait};
use crate::query::return_query::{ReturnQuery, ReturnTrait};

pub trait QueryTrait: 'static {
    fn create(&self) -> Box<dyn ReturnTrait>;
    fn r#match(&self) -> Box<dyn MatchTrait>;
}

pub struct Query {
    nv: String,
    data: Arc<Node>,
}

impl Query {
    pub fn new(nv: String, data: Arc<Node>) -> Self {
        Query { nv, data }
    }

    pub fn default(data: Arc<Node>) -> Self {
        Query::new(String::from("n"), data)
    }
}

impl QueryTrait for Query {
    fn create(&self) -> Box<dyn ReturnTrait> {
        let state = if self.data.props().len() > 0 {
            let mut props: String = self
                .data
                .props()
                .into_iter()
                .filter(|(_, v)| **v != PropType::Null)
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

        Box::new(ReturnQuery::new(self.nv.clone(), state))
    }

    fn r#match(&self) -> Box<dyn MatchTrait> {
        let state = format!("MATCH ({}:{})", self.nv, self.data.get_label(0).unwrap());
        Box::new(MatchQuery::new(self.nv.clone(), state))
    }
}
