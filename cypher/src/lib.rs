pub mod query;

#[cfg(feature = "derive")]
pub use cypher_derive::Cypherize;

pub trait ReturnTrait {
    fn finalize(&self) -> String;
}

pub trait CreateTrait {
    fn r#return(&mut self) -> Box<dyn ReturnTrait>;
}

pub trait QueryTrait {
    fn create(&mut self) -> Box<dyn CreateTrait>;
}

pub trait CypherTrait {
    fn cypher(&self) -> Box<dyn QueryTrait>;
}

// #[derive(Clone)]
// pub struct QueryContext {
//     /// Variable of current node.
//     /// Default is `n`
//     nv: String,
//     /// The current state of the request
//     state: String,
// }

// impl Default for QueryContext {
//     fn default() -> Self {
//         Self {
//             nv: String::from("n"),
//             state: String::new(),
//         }
//     }
// }

// impl QueryContext {
//     pub fn new(nv: String) -> Self {
//         Self {
//             nv,
//             state: String::new(),
//         }
//     }

//     pub fn state(&self) -> String {
//         self.state.clone()
//     }

//     pub fn push_to_state(&mut self, ns: &str) {
//         self.state.push_str(ns);
//     }

//     pub fn nv(&self) -> String {
//         self.nv.clone()
//     }

//     pub fn set_nv(&mut self, name: &str) {
//         self.nv = String::from(name);
//     }
// }

// pub trait BaseReadQuery {
//     fn r#match(&self) -> Query;
//     fn r#where(&self) -> Query;
//     fn r#return(&self) -> Query;
// }

// pub trait BaseWriteQuery {
//     fn create(&self);

//     fn set<'a, T>(&'a self, prop: &'a str, value: T)
//     where
//         T: ToString;

//     fn delete<'a, T>(&'a self, prop: &'a str, value: T)
//     where
//         T: ToString;

//     fn remove<'a, T>(&self, prop: &'a str, value: T)
//     where
//         T: ToString;
// }
