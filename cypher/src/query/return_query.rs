use crate::query::finalize::{Finalize, FinalizeTrait};

pub trait SkipControlTrait: 'static + FinalizeTrait {
    fn skip(&self, value: usize) -> Box<dyn FinalizeTrait>;
}

pub struct SkipControlQuery(String);

impl SkipControlTrait for SkipControlQuery {
    fn skip(&self, value: usize) -> Box<dyn FinalizeTrait> {
        let state = format!(
            "{prev_state}\nSKIP {value}",
            prev_state = self.0,
            value = value
        );
        Box::new(Finalize(state))
    }
}

impl FinalizeTrait for SkipControlQuery {
    fn finalize(&self) -> String {
        self.0.clone()
    }
}

pub trait LimitControlTrait: 'static + SkipControlTrait + FinalizeTrait {
    fn limit(&self, value: usize) -> Box<dyn SkipControlTrait>;
}

pub struct LimitControlQuery(String);

impl SkipControlTrait for LimitControlQuery {
    fn skip(&self, value: usize) -> Box<dyn FinalizeTrait> {
        let state = format!(
            "{prev_state}\nSKIP {value}",
            prev_state = self.0,
            value = value
        );
        Box::new(Finalize(state))
    }
}

impl LimitControlTrait for LimitControlQuery {
    fn limit(&self, value: usize) -> Box<dyn SkipControlTrait> {
        let state = format!(
            "{prev_state}\nLIMIT {value}",
            prev_state = self.0,
            value = value
        );
        Box::new(SkipControlQuery(state))
    }
}

impl FinalizeTrait for LimitControlQuery {
    fn finalize(&self) -> String {
        self.0.clone()
    }
}

pub trait ReturnParamTrait: 'static + LimitControlTrait + FinalizeTrait {
    fn r#as(&self, r#as: &str) -> Box<dyn LimitControlTrait>;
}

pub struct ReturnParamQuery {
    state: String,
}

impl ReturnParamQuery {
    pub fn new(state: String) -> Self {
        Self { state }
    }
}

impl LimitControlTrait for ReturnParamQuery {
    fn limit(&self, value: usize) -> Box<dyn SkipControlTrait> {
        let state = format!(
            "{prev_state}\nLIMIT {value}",
            prev_state = self.state,
            value = value
        );
        Box::new(SkipControlQuery(state))
    }
}

impl SkipControlTrait for ReturnParamQuery {
    fn skip(&self, value: usize) -> Box<dyn FinalizeTrait> {
        let state = format!(
            "{prev_state}\nSKIP {value}",
            prev_state = self.state,
            value = value
        );
        Box::new(Finalize(state))
    }
}

impl ReturnParamTrait for ReturnParamQuery {
    fn r#as(&self, r#as: &str) -> Box<dyn LimitControlTrait> {
        let state = format!(
            "{prev_state} AS {as_val}",
            prev_state = self.state,
            as_val = r#as
        );
        Box::new(LimitControlQuery(state))
    }
}

impl FinalizeTrait for ReturnParamQuery {
    fn finalize(&self) -> String {
        self.state.clone()
    }
}

pub trait ReturnTrait: 'static + FinalizeTrait {
    fn r#return(&mut self, nv: &str, field: Option<&str>) -> Box<dyn ReturnParamTrait>;
    fn return_many(&mut self, nvs: Vec<&str>) -> Box<dyn ReturnParamTrait>;
}

pub struct ReturnQuery {
    state: String,
}

impl ReturnQuery {
    pub fn new(state: String) -> Self {
        ReturnQuery { state }
    }
}

impl ReturnTrait for ReturnQuery {
    fn r#return(&mut self, nv: &str, field: Option<&str>) -> Box<dyn ReturnParamTrait> {
        return_method(&self.state, vec![nv], field)
    }

    fn return_many(&mut self, nvs: Vec<&str>) -> Box<dyn ReturnParamTrait> {
        return_method(&self.state, nvs, None)
    }
}

impl FinalizeTrait for ReturnQuery {
    fn finalize(&self) -> String {
        self.state.clone()
    }
}

pub(super) fn return_method(
    state: &str,
    nvs: Vec<&str>,
    field: Option<&str>,
) -> Box<dyn ReturnParamTrait> {
    let state = if nvs.len() > 1 {
        let mut state = format!("{prev_state}\nRETURN ", prev_state = state);
        state.push_str(&(nvs.iter().map(|nv| format!("{},", nv)).collect::<String>()));
        state.pop();

        state
    } else {
        if let Some(field) = field {
            format!(
                "{prev_state}\nRETURN {node_var}.{prop_name}",
                prev_state = state,
                node_var = nvs[0],
                prop_name = field
            )
        } else {
            format!(
                "{prev_state}\nRETURN {node_var}",
                prev_state = state,
                node_var = nvs[0]
            )
        }
    };

    Box::new(ReturnParamQuery::new(state))
}
