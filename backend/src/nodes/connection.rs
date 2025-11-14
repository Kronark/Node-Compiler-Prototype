use std::fmt::Display;
use crate::nodes::{data_type::DataType, vbi::VBI};

pub struct Connection {
    pub instance_id: VBI,
    pub socket_slot: VBI,
    pub type_: DataType
}

impl Connection {
    pub const fn new(i : VBI, s : VBI, t : DataType) -> Self {
        Self {
            instance_id: i,
            socket_slot: s,
            type_: t
        }
    }
}

impl Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Connection from {}, slot {}, {}",
            self.instance_id, self.socket_slot, self.type_
        )
    }
}
