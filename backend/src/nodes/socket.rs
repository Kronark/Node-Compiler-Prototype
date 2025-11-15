use std::fmt::Display;
use std::collections::HashSet;
use crate::nodes::connection::Connection;
use crate::nodes::socket_type::SocketType;
use crate::nodes::socket_parameters::SocketParameter;
use crate::nodes::data_value::DataValue;
use crate::nodes::data_type::DataType;
use crate::nodes::vbi::VBI;

// FIXME: overhaul socket slot as separate object with node space based assignment
pub struct Socket {
    pub is_outgoing: bool,
    pub is_repetition: bool,
    pub slot: VBI,
    pub type_: SocketType,
    pub parameters: SocketParameter,
    pub permitted: HashSet<DataType>,
    pub default_value: String,
    pub value: Option<DataValue>,
    pub connection: Option<Connection>
}

impl Socket {
    pub fn new(io : bool, ir : bool, s : VBI, t : SocketType, p : SocketParameter, d : String, v : Option<DataValue>, c : Option<Connection>) -> Self {
        Self {
            is_outgoing: io,
            is_repetition: ir,
            slot: s,
            type_: t,
            parameters: p,
            permitted: HashSet::new(),
            default_value: d,
            value: v,
            connection: c
        }
    }

    pub fn is_permitted(&self, query : DataType) -> bool {
        self.permitted.contains(&query)
    }
}

impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction : &str = if self.is_outgoing { "<" } else { ">" };
        let repetition : &str = if self.is_repetition { "↻" } else { "⁠—" };

        write!(
            f,
            "{} {} {} {}\n    PARAMETERS\n    PERMITTED\n    VALUE{}",
            direction, repetition, self.slot, self.type_,
            self.connection.as_ref().map_or("".to_owned(), |par| "\n    ".to_owned() + &par.to_string())
        )
    }
}
