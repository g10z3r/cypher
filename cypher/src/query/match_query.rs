use crate::entity::{Entity, PropType};
use crate::query::finalize::FinalizeTrait;
use crate::query::return_query::{ReturnParamTrait, ReturnQuery, ReturnTrait};
use crate::query::QueryTrait;

/// Comparison operators.
pub enum CompOper {
    /// Operation `=`.
    /// This means using the following construct in the query:
    /// `n.{prop} = {value}`
    Equal,
    /// Operation `>`.
    /// This means using the following construct in the query:
    /// `n.{prop} > {value}`
    More,
    /// Operation `<`.
    /// This means using the following construct in the query:
    /// `n.{prop} < {value}`
    Less,
    /// Operation `>=`.
    /// This means using the following construct in the query:
    /// `n.{prop} >= {value}`
    MoreEqual,
    /// Operation `<=`.
    /// This means using the following construct in the query:
    /// `n.{prop} <= {value}`
    LessEqual,
}

impl std::fmt::Display for CompOper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompOper::Equal => write!(f, "="),
            CompOper::More => write!(f, ">"),
            CompOper::Less => write!(f, "<"),
            CompOper::MoreEqual => write!(f, ">="),
            CompOper::LessEqual => write!(f, "<="),
        }
    }
}

pub trait MatchActionTrait: 'static + ReturnTrait {
    fn delete(&self) -> Box<dyn ReturnTrait>;
    fn delete_detach(&self) -> Box<dyn ReturnTrait>;

    fn set(&self, prop: &str, value: PropType) -> Box<dyn ReturnTrait>;

    fn set_str(&self, prop: &str, value: &str) -> Box<dyn ReturnTrait>;
    fn set_str_var(&self, nv: &str, prop: &str, value: &str) -> Box<dyn ReturnTrait>;

    fn set_int(&self, prop: &str, value: isize) -> Box<dyn ReturnTrait>;
    fn set_int_var(&self, nv: &str, prop: &str, value: isize) -> Box<dyn ReturnTrait>;

    fn set_bool(&self, prop: &str, value: bool) -> Box<dyn ReturnTrait>;
    fn set_bool_var(&self, nv: &str, prop: &str, value: bool) -> Box<dyn ReturnTrait>;

    fn set_var(&self, nv: &str, prop: &str, value: PropType) -> Box<dyn ReturnTrait>;
}

pub trait MatchConditionTrait: 'static + MatchActionTrait + QueryTrait {
    fn and(&mut self, prop: &str, op: CompOper, eq: PropType) -> Box<dyn MatchConditionTrait>;

    fn and_eq_str(&mut self, prop: &str, value: &str) -> Box<dyn MatchConditionTrait>;
    fn and_eq_str_var(&mut self, nv: &str, prop: &str, value: &str)
        -> Box<dyn MatchConditionTrait>;
    fn and_eq_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    fn and_eq_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait>;
    fn and_eq_bool(&mut self, prop: &str, value: bool) -> Box<dyn MatchConditionTrait>;
    fn and_eq_bool_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: bool,
    ) -> Box<dyn MatchConditionTrait>;

    fn and_more_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    fn and_more_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait>;

    fn and_less_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    fn and_less_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait>;

    fn and_moreq_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    fn and_moreq_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait>;

    fn and_leseq_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    fn and_leseq_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait>;

    fn and_var(
        &mut self,
        nv: &str,
        prop: &str,
        op: CompOper,
        eq: PropType,
    ) -> Box<dyn MatchConditionTrait>;

    fn or(&mut self, prop: &str, op: CompOper, eq: PropType) -> Box<dyn MatchConditionTrait>;

    fn or_eq_str(&mut self, prop: &str, value: &str) -> Box<dyn MatchConditionTrait>;
    fn or_eq_str_var(&mut self, nv: &str, prop: &str, value: &str) -> Box<dyn MatchConditionTrait>;

    fn or_eq_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    fn or_eq_int_var(&mut self, nv: &str, prop: &str, value: isize)
        -> Box<dyn MatchConditionTrait>;

    fn or_eq_bool(&mut self, prop: &str, value: bool) -> Box<dyn MatchConditionTrait>;
    fn or_eq_bool_var(&mut self, nv: &str, prop: &str, value: bool)
        -> Box<dyn MatchConditionTrait>;

    fn or_more_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    fn or_more_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait>;

    fn or_less_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    fn or_less_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait>;

    fn or_moreq_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    fn or_moreq_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait>;

    fn or_leseq_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    fn or_leseq_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait>;

    fn or_var(
        &mut self,
        nv: &str,
        prop: &str,
        op: CompOper,
        eq: PropType,
    ) -> Box<dyn MatchConditionTrait>;
}

pub struct MatchConditionQuery {
    nv: String,
    state: String,
}

impl MatchConditionQuery {
    pub fn new(nv: String, state: String) -> Self {
        MatchConditionQuery { nv, state }
    }
}

impl FinalizeTrait for MatchConditionQuery {
    fn finalize(&self) -> String {
        self.state.clone()
    }
}

impl ReturnTrait for MatchConditionQuery {
    fn r#return(&mut self, nv: &str, field: Option<&str>) -> Box<dyn ReturnParamTrait> {
        super::return_query::return_method(&self.state, vec![nv], field)
    }

    fn return_many(&mut self, nvs: Vec<&str>) -> Box<dyn ReturnParamTrait> {
        super::return_query::return_method(&self.state, nvs, None)
    }
}

impl MatchActionTrait for MatchConditionQuery {
    fn delete(&self) -> Box<dyn ReturnTrait> {
        let state = format!(
            "{prev_state}\nDELETE {node_var}\n",
            prev_state = self.state,
            node_var = self.nv
        );
        Box::new(ReturnQuery::new(state))
    }

    fn delete_detach(&self) -> Box<dyn ReturnTrait> {
        let state = format!(
            "{prev_state}\nDETACH DELETE {node_var}\n",
            prev_state = self.state,
            node_var = self.nv
        );
        Box::new(ReturnQuery::new(state))
    }

    fn set(&self, prop: &str, value: PropType) -> Box<dyn ReturnTrait> {
        let state = format!(
            "{prev_state}\nSET {node_var}.{prop_name}={value}",
            prev_state = self.state,
            node_var = self.nv,
            prop_name = prop,
            value = value.to_prop()
        );
        Box::new(ReturnQuery::new(state))
    }

    fn set_var(&self, nv: &str, prop: &str, value: PropType) -> Box<dyn ReturnTrait> {
        let state = format!(
            "{prev_state}\nSET {node_var}.{prop_name}={value}",
            prev_state = self.state,
            node_var = nv,
            prop_name = prop,
            value = value.to_prop()
        );
        Box::new(ReturnQuery::new(state))
    }

    fn set_str(&self, prop: &str, value: &str) -> Box<dyn ReturnTrait> {
        self.set(prop, PropType::str(value.to_string()))
    }

    fn set_str_var(&self, nv: &str, prop: &str, value: &str) -> Box<dyn ReturnTrait> {
        self.set_var(nv, prop, PropType::str(value.to_string()))
    }

    fn set_int(&self, prop: &str, value: isize) -> Box<dyn ReturnTrait> {
        self.set(prop, PropType::int(value))
    }

    fn set_int_var(&self, nv: &str, prop: &str, value: isize) -> Box<dyn ReturnTrait> {
        self.set_var(nv, prop, PropType::int(value))
    }

    fn set_bool(&self, prop: &str, value: bool) -> Box<dyn ReturnTrait> {
        self.set(prop, PropType::Bool(value))
    }

    fn set_bool_var(&self, nv: &str, prop: &str, value: bool) -> Box<dyn ReturnTrait> {
        self.set_var(nv, prop, PropType::Bool(value))
    }
}

impl MatchConditionTrait for MatchConditionQuery {
    fn and(&mut self, prop: &str, op: CompOper, eq: PropType) -> Box<dyn MatchConditionTrait> {
        let state = format!(
            "{prev_state} AND {node_var}.{prop_name} {operator} {value} ",
            prev_state = self.state,
            node_var = self.nv,
            prop_name = prop,
            operator = op.to_string(),
            value = eq.to_prop()
        );

        Box::new(Self::new(self.nv.clone(), state))
    }

    fn or(&mut self, prop: &str, op: CompOper, eq: PropType) -> Box<dyn MatchConditionTrait> {
        let state = format!(
            "{prev_state} OR {node_var}.{prop_name} {operator} {value} ",
            prev_state = self.state,
            node_var = self.nv,
            prop_name = prop,
            operator = op.to_string(),
            value = eq.to_prop()
        );
        Box::new(Self::new(self.nv.clone(), state))
    }

    fn and_var(
        &mut self,
        nv: &str,
        prop: &str,
        op: CompOper,
        eq: PropType,
    ) -> Box<dyn MatchConditionTrait> {
        let state = format!(
            "{prev_state} AND {node_var}.{prop_name} {operator} {value} ",
            prev_state = self.state,
            node_var = nv,
            prop_name = prop,
            operator = op.to_string(),
            value = eq.to_prop()
        );

        Box::new(Self::new(self.nv.clone(), state))
    }

    fn or_var(
        &mut self,
        nv: &str,
        prop: &str,
        op: CompOper,
        eq: PropType,
    ) -> Box<dyn MatchConditionTrait> {
        let state = format!(
            "{prev_state} OR {node_var}.{prop_name} {operator} {value} ",
            prev_state = self.state,
            node_var = nv,
            prop_name = prop,
            operator = op.to_string(),
            value = eq.to_prop()
        );
        Box::new(Self::new(self.nv.clone(), state))
    }

    fn and_eq_str(&mut self, prop: &str, value: &str) -> Box<dyn MatchConditionTrait> {
        self.and(prop, CompOper::Equal, PropType::str(value.to_string()))
    }

    fn and_eq_str_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: &str,
    ) -> Box<dyn MatchConditionTrait> {
        self.and_var(nv, prop, CompOper::Equal, PropType::str(value.to_string()))
    }

    fn and_eq_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.and(prop, CompOper::Equal, PropType::int(value))
    }

    fn and_eq_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait> {
        self.and_var(nv, prop, CompOper::Equal, PropType::int(value))
    }

    fn and_eq_bool(&mut self, prop: &str, value: bool) -> Box<dyn MatchConditionTrait> {
        self.and(prop, CompOper::Equal, PropType::Bool(value))
    }

    fn and_eq_bool_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: bool,
    ) -> Box<dyn MatchConditionTrait> {
        self.and_var(nv, prop, CompOper::Equal, PropType::Bool(value))
    }

    fn and_more_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.and(prop, CompOper::More, PropType::int(value))
    }

    fn and_more_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait> {
        self.and_var(nv, prop, CompOper::More, PropType::int(value))
    }

    fn and_less_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.and(prop, CompOper::Less, PropType::int(value))
    }

    fn and_less_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait> {
        self.and_var(nv, prop, CompOper::Less, PropType::int(value))
    }

    fn and_moreq_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.and(prop, CompOper::MoreEqual, PropType::int(value))
    }

    fn and_moreq_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait> {
        self.and_var(nv, prop, CompOper::MoreEqual, PropType::int(value))
    }

    fn and_leseq_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.and(prop, CompOper::LessEqual, PropType::int(value))
    }

    fn and_leseq_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait> {
        self.and_var(nv, prop, CompOper::LessEqual, PropType::int(value))
    }

    fn or_eq_str(&mut self, prop: &str, value: &str) -> Box<dyn MatchConditionTrait> {
        self.or(prop, CompOper::Equal, PropType::str(value.to_string()))
    }

    fn or_eq_str_var(&mut self, nv: &str, prop: &str, value: &str) -> Box<dyn MatchConditionTrait> {
        self.or_var(nv, prop, CompOper::Equal, PropType::str(value.to_string()))
    }

    fn or_eq_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.or(prop, CompOper::Equal, PropType::int(value))
    }

    fn or_eq_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait> {
        self.or_var(nv, prop, CompOper::Equal, PropType::int(value))
    }

    fn or_eq_bool(&mut self, prop: &str, value: bool) -> Box<dyn MatchConditionTrait> {
        self.or(prop, CompOper::Equal, PropType::Bool(value))
    }

    fn or_eq_bool_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: bool,
    ) -> Box<dyn MatchConditionTrait> {
        self.or_var(nv, prop, CompOper::Equal, PropType::Bool(value))
    }

    fn or_more_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.or(prop, CompOper::More, PropType::int(value))
    }

    fn or_more_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait> {
        self.or_var(nv, prop, CompOper::More, PropType::int(value))
    }

    fn or_less_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.or(prop, CompOper::Less, PropType::int(value))
    }

    fn or_less_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait> {
        self.or_var(nv, prop, CompOper::Less, PropType::int(value))
    }

    fn or_moreq_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.or(prop, CompOper::MoreEqual, PropType::int(value))
    }

    fn or_moreq_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait> {
        self.or_var(nv, prop, CompOper::MoreEqual, PropType::int(value))
    }

    fn or_leseq_int(&mut self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.or(prop, CompOper::LessEqual, PropType::int(value))
    }

    fn or_leseq_int_var(
        &mut self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait> {
        self.or_var(nv, prop, CompOper::LessEqual, PropType::int(value))
    }
}

impl QueryTrait for MatchConditionQuery {
    fn create(&mut self, entitys: Vec<&Entity>) -> Box<dyn ReturnTrait> {
        super::create_method(&mut self.state, entitys)
    }

    fn r#match(&mut self, entity: &Entity, optional: bool) -> Box<dyn MatchTrait> {
        super::match_method(&mut self.state, entity, optional)
    }
}

pub trait MatchTrait: 'static {
    /// Pure **WHERE** query function.  
    ///
    /// Mostly used in internal methods that form shorter and more specialized functions
    /// or if you want to use types other than `String`, `Int` `Bool`.
    fn r#where(&self, prop: &str, op: CompOper, eq: PropType) -> Box<dyn MatchConditionTrait>;
    /// Pure **WHERE** query function with custom var.  
    ///
    /// Mostly used in internal methods that form shorter and more specialized functions
    /// or if you want to use types other than `String`, `Int` `Bool`.
    fn where_var(
        &self,
        nv: &str,
        prop: &str,
        op: CompOper,
        eq: PropType,
    ) -> Box<dyn MatchConditionTrait>;

    /// A short use case for the where function, assuming the following final result:
    ///
    /// `WHERE n.prop = '...'`
    fn where_eq_str(&self, prop: &str, value: &str) -> Box<dyn MatchConditionTrait>;
    /// A short use case for the where function with custom var, assuming the following final result:
    ///
    /// `WHERE n.prop = '...'`
    fn where_eq_str_var(&self, nv: &str, prop: &str, value: &str) -> Box<dyn MatchConditionTrait>;
    /// A short use case for the where function, assuming the following final result:
    ///
    /// `WHERE n.prop = 0`
    fn where_eq_int(&self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    /// A short use case for the where function with custom var, assuming the following final result:
    ///
    /// `WHERE n.prop = 0`
    fn where_eq_int_var(&self, nv: &str, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    /// A short use case for the where function, assuming the following final result:
    ///
    /// `WHERE n.prop = true`
    fn where_eq_bool(&self, prop: &str, value: bool) -> Box<dyn MatchConditionTrait>;
    /// A short use case for the where function with custom var, assuming the following final result:
    ///
    /// `WHERE n.prop = true`
    fn where_eq_bool_var(&self, nv: &str, prop: &str, value: bool) -> Box<dyn MatchConditionTrait>;
    /// A short use case for the where function, assuming the following final result:
    ///
    /// `WHERE n.prop > 0`
    fn where_more_int(&self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    /// A short use case for the where function with custom var, assuming the following final result:
    ///
    /// `WHERE n.prop > 0`
    fn where_more_int_var(
        &self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait>;
    /// A short use case for the where function, assuming the following final result:
    ///
    /// `WHERE n.prop < 0`
    fn where_less_int(&self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    /// A short use case for the where function with custom var, assuming the following final result:
    ///
    /// `WHERE n.prop < 0`
    fn where_less_int_var(
        &self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait>;
    /// A short use case for the where function, assuming the following final result:
    ///
    /// `WHERE n.prop >= 0`
    fn where_moreq_int(&self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    /// A short use case for the where function with custom var, assuming the following final result:
    ///
    /// `WHERE n.prop >= 0`
    fn where_moreq_int_var(
        &self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait>;
    /// A short use case for the where function, assuming the following final result:
    ///
    /// `WHERE n.prop <= 0`
    fn where_leseq_int(&self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait>;
    /// A short use case for the where function with custom var, assuming the following final result:
    ///
    /// `WHERE n.prop <= 0`
    fn where_leseq_int_var(
        &self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait>;
}

pub struct MatchQuery {
    nv: String,
    state: String,
}

impl MatchQuery {
    pub fn new(nv: String, state: String) -> Self {
        MatchQuery { nv, state }
    }
}

impl MatchTrait for MatchQuery {
    fn r#where(&self, prop: &str, op: CompOper, eq: PropType) -> Box<dyn MatchConditionTrait> {
        let state = format!(
            "{prev_state} WHERE {node_var}.{prop_name} {operator} {value}",
            prev_state = self.state,
            node_var = self.nv,
            prop_name = prop,
            operator = op,
            value = eq.to_prop()
        );
        Box::new(MatchConditionQuery::new(self.nv.clone(), state))
    }

    /* Equal */

    fn where_eq_str(&self, prop: &str, value: &str) -> Box<dyn MatchConditionTrait> {
        self.r#where(prop, CompOper::Equal, PropType::str(value.to_string()))
    }
    fn where_eq_str_var(&self, nv: &str, prop: &str, value: &str) -> Box<dyn MatchConditionTrait> {
        self.where_var(nv, prop, CompOper::Equal, PropType::str(value.to_string()))
    }
    fn where_eq_int(&self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.r#where(prop, CompOper::Equal, PropType::int(value))
    }
    fn where_eq_int_var(&self, nv: &str, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.where_var(nv, prop, CompOper::Equal, PropType::int(value))
    }
    fn where_eq_bool(&self, prop: &str, value: bool) -> Box<dyn MatchConditionTrait> {
        self.r#where(prop, CompOper::Equal, PropType::Bool(value))
    }
    fn where_eq_bool_var(&self, nv: &str, prop: &str, value: bool) -> Box<dyn MatchConditionTrait> {
        self.where_var(nv, prop, CompOper::Equal, PropType::Bool(value))
    }

    /* More */

    fn where_more_int(&self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.r#where(prop, CompOper::More, PropType::int(value))
    }
    fn where_more_int_var(
        &self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait> {
        self.where_var(nv, prop, CompOper::More, PropType::int(value))
    }

    /* Less */

    fn where_less_int(&self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.r#where(prop, CompOper::Less, PropType::int(value))
    }
    fn where_less_int_var(
        &self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait> {
        self.where_var(nv, prop, CompOper::Less, PropType::int(value))
    }

    /* MoreEqual */

    fn where_moreq_int(&self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.r#where(prop, CompOper::MoreEqual, PropType::int(value))
    }
    fn where_moreq_int_var(
        &self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait> {
        self.where_var(nv, prop, CompOper::MoreEqual, PropType::int(value))
    }

    /* LessEqual */

    fn where_leseq_int(&self, prop: &str, value: isize) -> Box<dyn MatchConditionTrait> {
        self.r#where(prop, CompOper::LessEqual, PropType::int(value))
    }

    fn where_leseq_int_var(
        &self,
        nv: &str,
        prop: &str,
        value: isize,
    ) -> Box<dyn MatchConditionTrait> {
        self.where_var(nv, prop, CompOper::LessEqual, PropType::int(value))
    }

    fn where_var(
        &self,
        nv: &str,
        prop: &str,
        op: CompOper,
        eq: PropType,
    ) -> Box<dyn MatchConditionTrait> {
        let state = format!(
            "{prev_state} WHERE {node_var}.{prop_name} {operator} {value}",
            prev_state = self.state,
            node_var = nv,
            prop_name = prop,
            operator = op,
            value = eq.to_prop()
        );
        Box::new(MatchConditionQuery::new(self.nv.clone(), state))
    }
}
