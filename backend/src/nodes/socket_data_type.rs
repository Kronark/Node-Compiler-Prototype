use std::fmt::Display;
use std::collections::HashSet;
use crate::nodes::connection_data_type::Connection;
use crate::nodes::type_data_type::Type;

pub enum SocketType
{
    Named,
    Number,
    Select,
    Switch,
    Text
}

impl Display for SocketType
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            SocketType::Named => write!(f, "named"),
            SocketType::Number => write!(f, "number"),
            SocketType::Select => write!(f, "select"),
            SocketType::Switch => write!(f, "switch"),
            SocketType::Text => write!(f, "text")
        }
    }
}

pub struct Socket
{
    pub is_outgoing: bool,
    pub is_repetition: bool,
    pub slot: u32,
    pub type_: SocketType,
    pub parameters: u8,
    pub permitted: HashSet<Type>,
    pub value: u8,
    pub connection: Option<Connection>
}

impl Socket
{
    pub fn new(io : bool, ir : bool, s : u32, t : SocketType, c : Option<Connection>) -> Self
    {
        Self {
            is_outgoing: io,
            is_repetition: ir,
            slot: s,
            type_: t,
            parameters: 0,
            permitted: HashSet::new(),
            value: 0,
            connection: c
        }
    }

    pub fn is_permitted() { todo!("Permission check missing") }
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
