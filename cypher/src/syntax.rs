use std::{collections::HashMap, fmt::format};

use crate::query::{self, Query};

// pub trait CreateTrait {
//     fn r#return(&self) -> Box<dyn ReturnTrait>;
// }

// pub trait ReturnTrait {
//     fn finalize(&self) -> String;
// }

// pub trait QueryTrait {
//     fn create(
//         &mut self,
//         node_name: String,
//         props: Option<HashMap<String, String>>,
//     ) -> Box<dyn CreateTrait>;
// }

// pub trait CypherQuery {
//     fn cypher(&self) -> Box<dyn QueryTrait>;
// }

// impl QueryTrait for Query {
//     fn create(
//         &mut self,
//         node_name: String,
//         props: Option<HashMap<String, String>>,
//     ) -> Box<dyn CreateTrait> {
//         if let Some(props) = props {
//             let mut props: String = props
//                 .into_iter()
//                 .map(|(k, v)| format!("{}: '{}',", k, v))
//                 .collect();
//             props.pop();

//             self.push_to_state(&format!(
//                 "CREATE ({}:{} {{ {} }})",
//                 self.nv(),
//                 node_name,
//                 props
//             ))
//         } else {
//             self.push_to_state(&format!("CREATE ({}:{})", self.nv(), node_name))
//         }

//         Box::new(self.clone())
//     }
// }

