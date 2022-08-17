use crate::node::PropType;
use crate::query::execute::{Execute, ExecuteTrait};
use crate::query::return_query::{ReturnQuery, ReturnTrait};

pub trait MatchActionTrait: 'static + ReturnTrait {
    fn delete(&self) -> Box<dyn ReturnTrait>;
    fn delete_detach(&self) -> Box<dyn ReturnTrait>;
}

pub trait MatchConditionTrait: 'static + MatchActionTrait {
    fn and(&mut self, prop: &str, eq: PropType) -> Box<dyn MatchConditionTrait>;
    fn or(&mut self, prop: &str, eq: PropType) -> Box<dyn MatchConditionTrait>;
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
    fn r#return(&mut self) -> Box<dyn ExecuteTrait> {
        let state = format!("{}\nRETURN {}", self.state, self.nv);
        Box::new(Execute(state))
    }

    fn return_as(&mut self, r#as: &str) -> Box<dyn ExecuteTrait> {
        let state = format!("{}\nRETURN {} AS {}", self.state, self.nv, r#as);
        Box::new(Execute(state))
    }

    fn return_field(&mut self, field: &str) -> Box<dyn ExecuteTrait> {
        let state = format!("{}\nRETURN {}.{}", self.state, self.nv, field);
        Box::new(Execute(state))
    }
}

impl MatchActionTrait for MatchConditionQuery {
    fn delete(&self) -> Box<dyn ReturnTrait> {
        let state = format!("{}\nDELETE {}\n", self.state, self.nv);
        Box::new(ReturnQuery::new(self.nv.clone(), state))
    }

    fn delete_detach(&self) -> Box<dyn ReturnTrait> {
        let state = format!("{}\nDETACH DELETE {}\n", self.state, self.nv);
        Box::new(ReturnQuery::new(self.nv.clone(), state))
    }
}

impl MatchConditionTrait for MatchConditionQuery {
    fn and(&mut self, prop: &str, eq: PropType) -> Box<dyn MatchConditionTrait> {
        let state = format!(
            "{} AND {}.{} = {} ",
            self.state,
            self.nv,
            prop,
            eq.to_prop()
        );

        Box::new(Self::new(self.nv.clone(), state))
    }

    fn or(&mut self, prop: &str, eq: PropType) -> Box<dyn MatchConditionTrait> {
        let state = format!("{} OR {}.{} = {} ", self.state, self.nv, prop, eq.to_prop());
        Box::new(Self::new(self.nv.clone(), state))
    }
}

pub trait MatchTrait: 'static {
    fn r#where(&self, prop: &str, eq: PropType) -> Box<dyn MatchConditionTrait>;
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
    fn r#where(&self, prop: &str, eq: PropType) -> Box<dyn MatchConditionTrait> {
        let state = format!(
            "{}\nWHERE {}.{} = {}",
            self.state,
            self.nv,
            prop,
            eq.to_prop()
        );
        Box::new(MatchConditionQuery::new(self.nv.clone(), state))
    }
}
