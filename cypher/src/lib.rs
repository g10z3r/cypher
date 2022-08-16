pub mod node;
pub mod query;

use std::collections::HashMap;

#[cfg(feature = "derive")]
pub use cypher_derive::Cypherize;

use node::{Node, Props};
use query::{Query, QueryTrait};

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
