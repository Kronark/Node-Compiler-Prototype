use crate::nodes::data_type::DataType;
use std::{fmt::Display, sync::Arc};

pub struct Connection {
    pub instance_id: u32,
    pub socket_slot: u32,
    pub type_: Arc<DataType>,
}

impl Connection {
    pub const fn new(i: u32, s: u32, t: Arc<DataType>) -> Self {
        Self {
            instance_id: i,
            socket_slot: s,
            type_: t,
        }
    }
}

impl Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "instance {}, slot {}, {}",
            self.instance_id, self.socket_slot, self.type_
        )
    }
}

#[macro_export]
macro_rules! connection {
    ($instance_id:expr, $socket_slot:expr, $type_:expr) => {{ $crate::nodes::connection::Connection::new($instance_id, $socket_slot, $type_) }};
}
