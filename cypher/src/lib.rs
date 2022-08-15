pub mod query;

#[cfg(feature = "derive")]
pub use cypher_derive::Cypherize;

pub trait ReturnTrait {
    fn finalize(&self) -> String;
}

pub trait MatchTrait {}

pub trait WriteTrait: ReturnTrait {
    fn r#return(&mut self) -> Box<dyn ReturnTrait>;
    fn return_as(&mut self, value: &str) -> Box<dyn ReturnTrait>;
}

pub trait QueryTrait: WriteTrait {
    fn create(&mut self) -> Box<dyn WriteTrait>;
    fn delete(&mut self, detach: bool) -> Box<dyn WriteTrait>;
    // fn r#match(&mut self) -> Box<dyn MatchTrait>;
}
