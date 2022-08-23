use crate::entity::PropType;
use crate::query::finalize::{Finalize, FinalizeTrait};
use crate::query::return_query::{ReturnParamQuery, ReturnParamTrait, ReturnQuery, ReturnTrait};

/// Comparison Operators
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
}

pub trait MatchConditionTrait: 'static + MatchActionTrait {
    fn and(&mut self, prop: &str, op: CompOper, eq: PropType) -> Box<dyn MatchConditionTrait>;
    fn or(&mut self, prop: &str, op: CompOper, eq: PropType) -> Box<dyn MatchConditionTrait>;
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

impl ReturnTrait for MatchConditionQuery {
    fn r#return(&mut self, nv: &str) -> Box<dyn ReturnParamTrait> {
        let state = format!(
            "{prev_state}\nRETURN {node_var}",
            prev_state = self.state,
            node_var = nv
        );
        Box::new(ReturnParamQuery::new(self.nv.clone(), state))
    }

    fn return_field(&mut self, nv: &str, field: &str) -> Box<dyn FinalizeTrait> {
        let state = format!(
            "{prev_state}\nRETURN {node_var}.{prop_name}",
            prev_state = self.state,
            node_var = nv,
            prop_name = field
        );
        Box::new(Finalize(state))
    }
}

impl MatchActionTrait for MatchConditionQuery {
    fn delete(&self) -> Box<dyn ReturnTrait> {
        let state = format!(
            "{prev_state}\nDELETE {node_var}\n",
            prev_state = self.state,
            node_var = self.nv
        );
        Box::new(ReturnQuery::new(self.nv.clone(), state))
    }

    fn delete_detach(&self) -> Box<dyn ReturnTrait> {
        let state = format!(
            "{prev_state}\nDETACH DELETE {node_var}\n",
            prev_state = self.state,
            node_var = self.nv
        );
        Box::new(ReturnQuery::new(self.nv.clone(), state))
    }

    fn set(&self, prop: &str, value: PropType) -> Box<dyn ReturnTrait> {
        let state = format!(
            "{prev_state}\nSET {node_var}.{prop_name}={value}",
            prev_state = self.state,
            node_var = self.nv,
            prop_name = prop,
            value = value.to_prop()
        );
        Box::new(ReturnQuery::new(self.nv.clone(), state))
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
}

pub trait MatchTrait: 'static {
    fn r#where(&self, prop: &str, op: CompOper, eq: PropType) -> Box<dyn MatchConditionTrait>;
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
            "{prev_state}\nWHERE {node_var}.{prop_name} {operator} {value}",
            prev_state = self.state,
            node_var = self.nv,
            prop_name = prop,
            operator = op,
            value = eq.to_prop()
        );
        Box::new(MatchConditionQuery::new(self.nv.clone(), state))
    }
}
