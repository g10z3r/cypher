pub mod finalize;
pub mod match_query;
pub mod return_query;

use crate::entity::Entity;
use crate::entity::PropType;
use crate::entity::Props;
use crate::query::match_query::{MatchQuery, MatchTrait};
use crate::query::return_query::{ReturnQuery, ReturnTrait};

pub trait QueryTrait: 'static {
    fn create(&mut self, entitys: Vec<&Entity>) -> Box<dyn ReturnTrait>;

    fn r#match(&mut self, entity: &Entity, optional: bool) -> Box<dyn MatchTrait>;
}

pub struct Query {
    state: String,
}

impl Query {
    pub fn init() -> Self {
        Query {
            state: String::new(),
        }
    }

    pub fn new(state: String) -> Self {
        Query { state }
    }
}

impl QueryTrait for Query {
    fn create(&mut self, entitys: Vec<&Entity>) -> Box<dyn ReturnTrait> {
        create_method(&mut self.state, entitys)
    }

    fn r#match(&mut self, entity: &Entity, optional: bool) -> Box<dyn MatchTrait> {
        match_method(&mut self.state, entity, optional)
    }
}

pub(super) fn match_method(
    state: &mut str,
    entity: &Entity,
    optional: bool,
) -> Box<dyn MatchTrait> {
    match entity {
        Entity::Node { nv, node_name, .. } => {
            let new_state = format!(
                "MATCH ({node_var}:{node_name})",
                node_var = nv,
                node_name = node_name
            );

            let state = format!(
                "{state}{indent}{new_state}",
                state = state,
                indent = if state.len() > 0 { "\n" } else { "" },
                new_state = new_state
            );

            Box::new(MatchQuery::new(nv.to_string(), state.to_string()))
        }

        Entity::Relation { from, to, name, .. } => {
            let new_state = format!(
                "{opt}MATCH ({from_nv}{from_name})-[r:{rel_name}]->({to_nv}{to_name})",
                opt = if optional {
                    format!("OPTIONAL ")
                } else {
                    String::new()
                },
                from_nv = from.nv(),
                from_name = format!(":{}", from.node_name()),
                rel_name = name,
                to_nv = to.nv(),
                to_name = format!(":{}", to.node_name()),
            );

            let state = format!(
                "{state}{indent}{new_state}",
                state = state,
                indent = if state.len() > 0 { "\n" } else { "" },
                new_state = new_state
            );

            Box::new(MatchQuery::new(String::new(), state.to_string()))
        }
    }
}

pub(super) fn create_method(state: &mut str, entitys: Vec<&Entity>) -> Box<dyn ReturnTrait> {
    let new_state = entitys
        .iter()
        .enumerate()
        .map(|(i, entity)| match entity {
            Entity::Node {
                nv,
                node_name,
                props,
                labels,
            } => {
                let labels = if let Some(labels) = labels {
                    labels
                        .iter()
                        .map(|label| {
                            format!(
                                "\nSET {node_var}:{label_name}",
                                node_var = nv,
                                label_name = label.to_string()
                            )
                        })
                        .collect::<String>()
                } else {
                    String::new()
                };

                if let Some(props) = props {
                    format!(
                        "{indent}CREATE ({node_var}:{node_name} {{ {props_obj} }}){labels}",
                        indent = if i != 0 { "\n" } else { "" },
                        node_var = nv,
                        node_name = node_name,
                        props_obj = props_to_string(props),
                        labels = labels,
                    )
                } else {
                    format!(
                        "{indent}CREATE ({node_var}:{node_name}){labels}",
                        indent = if i != 0 { "\n" } else { "" },
                        node_var = nv,
                        node_name = node_name,
                        labels = labels,
                    )
                }
            }

            Entity::Relation {
                from,
                to,
                name,
                props,
            } => {
                if let Some(props) = props {
                    format!(
                        "{start_q}({from_nv})-[:{rel_name} {{ {props_obj} }}]->({to_nv}){is_next}",
                        start_q = if i == 0 { "CREATE " } else { "" },
                        from_nv = from.nv(),
                        rel_name = name,
                        props_obj = props_to_string(props),
                        to_nv = to.nv(),
                        is_next = if i == entitys.len() { "," } else { "" }
                    )
                } else {
                    format!(
                        "{start_q}({from_nv})-[:{rel_name}]->({to_nv}){is_next}",
                        start_q = if i == 0 { "CREATE " } else { "" },
                        from_nv = from.nv(),
                        rel_name = name,
                        to_nv = to.nv(),
                        is_next = if i < entitys.len() - 1 { ",\n\t" } else { "" }
                    )
                }
            }
        })
        .collect::<String>();

    let state = format!(
        "{state}{indent}{new_state}",
        state = state,
        indent = if state.len() > 0 { "\n" } else { "" },
        new_state = new_state
    );
    Box::new(ReturnQuery::new(state.to_string()))
}

fn props_to_string(props: &Props) -> String {
    let mut props: String = props
        .into_iter()
        .filter(|(_, v)| **v != PropType::Null)
        .map(|(k, v)| format!("{}: {},", k, v.to_prop()))
        .collect();
    props.pop();

    props
}
