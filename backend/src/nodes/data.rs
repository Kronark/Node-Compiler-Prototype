use std::{fmt::Display, sync::Arc};
use crate::nodes::{data_type::DataType, data_value::DataValue};

pub struct Data {
    pub type_: Arc<DataType>,
    pub value: DataValue,
}

impl Data {
    pub fn new(dt: Arc<DataType>, v: DataValue) -> Self {
        Self {
            type_: dt,
            value: v
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.type_, self.value)
    }
}

#[macro_export]
macro_rules! data {
    ($type:expr, $value:expr) => {
        $crate::nodes::data::Data::new($type, $value)
    };
}
