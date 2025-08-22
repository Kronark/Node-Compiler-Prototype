use std::fmt::Display;
use crate::nodes::r#type::Type;

pub struct Connection
{
    pub instance_id: u32,
    pub socket_slot: u32,
    pub type_: Type
}

impl Connection
{
    pub const fn new(i : u32, s : u32, t : Type) -> Self
    {
        Self {
            instance_id: i,
            socket_slot: s,
            type_: t
        }
    }
}

impl Display for Connection
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "Connection from {}, slot {}, {}",
            self.instance_id, self.socket_slot, self.type_
        )
    }
}
