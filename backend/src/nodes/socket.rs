use std::fmt::Display;
use std::collections::HashSet;
use crate::nodes::connection::Connection;
use crate::nodes::socket_type::SocketType;
use crate::nodes::socket_parameters::SocketParameter;
use crate::nodes::value::Value;
use crate::nodes::r#type::Type;

pub struct Socket
{
    pub is_outgoing: bool,
    pub is_repetition: bool,
    pub slot: u32,
    pub type_: SocketType,
    pub parameters: SocketParameter,
    pub permitted: HashSet<Type>,
    pub value: Option<Value>,
    pub connection: Option<Connection>
}

impl Socket
{
    pub fn new(io : bool, ir : bool, s : u32, t : SocketType, p : SocketParameter, v : Option<Value>, c : Option<Connection>) -> Self
    {
        Self {
            is_outgoing: io,
            is_repetition: ir,
            slot: s,
            type_: t,
            parameters: p,
            permitted: HashSet::new(),
            value: v,
            connection: c
        }
    }

    pub fn is_permitted(&self, query : Type) -> bool
    {
        self.permitted.contains(&query)
    }
}

impl Display for Socket
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let direction : &str = if self.is_outgoing { "<" } else { ">" };
        let repetition : &str = if self.is_repetition { "↻" } else { "⁠—" };

        write!(
            f,
            "{} {} {} {}\n    PARAMETERS\n    PERMITTED\n    VALUE\n    CONNECTION",
            direction, repetition, self.slot, self.type_
        )
    }
}
