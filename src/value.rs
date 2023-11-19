use std::collections::hash_set::Union;

#[derive(Clone)]
pub(crate) struct Value {
    pub(crate) value_type: ValueType,
    pub(crate) val: Val,
}

#[derive(Clone, Copy)]
pub(crate) union Val {
    pub(crate) i: i32,
    pub(crate) f: f64,
}

#[derive(Clone)]
pub(crate) enum ValueType {
    Int, Float,
}
