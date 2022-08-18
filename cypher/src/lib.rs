pub mod node;
pub mod query;

#[cfg(feature = "derive")]
pub use cypher_derive::CypQue;

use query::QueryTrait;

pub trait CypherTrait: 'static + Sized {
    fn cypher(&self) -> Box<dyn QueryTrait>;
}
