pub mod finalize;
pub mod match_query;
pub mod return_query;

use crate::entity::Entity;
use crate::entity::PropType;
use crate::query::match_query::{MatchQuery, MatchTrait};
use crate::query::return_query::{ReturnQuery, ReturnTrait};

pub trait QueryTrait: 'static {
    fn create(&mut self, entitys: Vec<Entity>) -> Box<dyn ReturnTrait> {
        create_method(entitys)
    }

    fn r#match(&self, entity: Entity) -> Box<dyn MatchTrait> {
        match_method(entity)
    }
}

pub struct Query;

impl Query {
    pub fn new() -> Self {
        Query
    }
}

impl QueryTrait for Query {}

pub(super) fn match_method(entity: Entity) -> Box<dyn MatchTrait> {
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

pub(super) fn create_method(entitys: Vec<Entity>) -> Box<dyn ReturnTrait> {
    let state = entitys
        .iter()
        .enumerate()
        .map(|(i, entity)| match entity {
            Entity::Node {
                nv,
                node_name,
                props,
                labels,
            } => {
                if props.len() > 0 {
                    let mut props: String = props
                        .into_iter()
                        .filter(|(_, v)| **v != PropType::Null)
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
                        "{indent}CREATE ({node_var}:{node_name} {{ {props_obj} }}){labels}",
                        indent = if i != 0 { "\n" } else { "" },
                        node_var = nv,
                        node_name = node_name,
                        props_obj = props,
                        labels = labels,
                    )
                } else {
                    format!(
                        "{indent}CREATE ({node_var}:{node_name})",
                        indent = if i != 0 { "\n" } else { "" },
                        node_var = nv,
                        node_name = node_name
                    )
                }
            }

            Entity::Relation => todo!(),
        })
        .collect::<String>();

    Box::new(ReturnQuery::new(state))
}
