pub mod finalize;
pub mod match_query;
pub mod return_query;

use crate::entity::Entity;
use crate::entity::PropType;
use crate::query::match_query::{MatchQuery, MatchTrait};
use crate::query::return_query::{ReturnQuery, ReturnTrait};

pub trait QueryTrait: 'static {
    fn create(&self, entity: Entity) -> Box<dyn ReturnTrait>;
    fn r#match(&self, entity: Entity) -> Box<dyn MatchTrait>;
}

pub struct Query;

impl Query {
    pub fn new() -> Self {
        Query
    }
}

impl QueryTrait for Query {
    fn create(&self, entity: Entity) -> Box<dyn ReturnTrait> {
        match entity {
            Entity::Node {
                nv,
                node_name,
                props,
                labels,
            } => {
                let state = if props.len() > 0 {
                    let mut props: String = props
                        .into_iter()
                        .filter(|(_, v)| *v != PropType::Null)
                        .map(|(k, v)| format!("{}: {},", k, v.to_prop()))
                        .collect();
                    props.pop();

                    let labels: String = labels
                        .iter()
                        .map(|label| {
                            format!(
                                "\nSET {node_var}:{label_name}",
                                node_var = nv,
                                label_name = label.to_string()
                            )
                        })
                        .collect();

                    format!(
                        "CREATE ({node_var}:{node_name} {{ {props_obj} }}){labels}",
                        node_var = nv,
                        node_name = node_name,
                        props_obj = props,
                        labels = labels,
                    )
                } else {
                    format!(
                        "CREATE ({node_var}:{node_name})",
                        node_var = nv,
                        node_name = node_name
                    )
                };

                Box::new(ReturnQuery::new(nv.to_string(), state))
            }
            Entity::Relation => todo!(),
        }
    }

    fn r#match(&self, entity: Entity) -> Box<dyn MatchTrait> {
        match entity {
            Entity::Node { nv, node_name, .. } => {
                let state = format!(
                    "MATCH ({node_var}:{node_name})",
                    node_var = nv,
                    node_name = node_name
                );
                Box::new(MatchQuery::new(nv.to_string(), state))
            }
            Entity::Relation => todo!(),
        }
    }
}
