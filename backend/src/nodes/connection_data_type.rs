use crate::nodes::type_data_type::Type;

pub struct Connection
{
    pub instance_id: u32,
    pub socket_slot: u32,
    pub type_: Type
}

impl Connection
{
    pub fn new(i : u32, s : u32, t : Type) -> Self
    {
        Self {
            instance_id: i,
            socket_slot: s,
            type_: t
        }
    }

    pub fn print(&self) { println!("Connection from {}, slot {}, type {}", self.instance_id, self.socket_slot, self.type_.identifier) }
}
