use std::fmt::Display;
use std::collections::HashSet;
use crate::nodes::connection::Connection;
use crate::nodes::socket_type::SocketType;
use crate::nodes::socket_parameters::SocketParameters;
use crate::nodes::data_value::DataValue;
use crate::nodes::data_type::DataType;

// FIXME: overhaul socket slot as separate object with node space based assignment
pub struct Socket {
    pub is_outgoing: bool,
    pub is_repetition: bool,
    pub slot: u32,
    pub type_: SocketType,
    pub parameters: SocketParameters,
    permitted: HashSet<DataType>,
    pub default_value: DataValue,
    pub value: Option<DataValue>,
    pub connection: Option<Connection>
}

impl Socket {
    pub fn new(
        io: bool,
        ir: bool,
        s: u32,
        t: SocketType,
        param: SocketParameters,
        perm: &[DataType],
        d: DataValue,
        v: Option<DataValue>,
        c: Option<Connection>,
    ) -> Self {
        Self {
            is_outgoing: io,
            is_repetition: ir,
            slot: s,
            type_: t,
            parameters: param,
            permitted: perm.iter().cloned().collect(),
            default_value: d,
            value: v,
            connection: c
        }
    }

    pub fn permit(&mut self, data_type: DataType) {
        self.permitted.insert(data_type);
    }

    pub fn forbid(&mut self, data_type: &DataType) {
        self.permitted.remove(data_type);
    }

    pub fn is_permitted(&self, query : DataType) -> bool {
        self.permitted.contains(&query)
    }
}

impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction : &str = if self.is_outgoing { "<" } else { ">" };
        let repetition : &str = if self.is_repetition { "↻" } else { "⁠—" };

        let permitted_string = if self.permitted.is_empty() {
            "no permitted types".to_string()
        } else {
            let bullet_points = self.permitted
                .iter()
                .map(|dt| format!("    • {}", dt))
                .collect::<Vec<_>>()
                .join("\n");
            format!("permitted:\n{}", bullet_points)
        };

        let value_string = match &self.value {
            Some(value) => value.to_string(),
            None => "no value".to_owned(),
        };

        let connection_string = match &self.connection {
            Some(conn) => conn.to_string(),
            None => "no connection".to_owned(),
        };

        write!(
            f,
            "{} {} {} {}\n{}\n{}\ndefault: {}\nvalue: {}\nconnection: {}",
            direction, repetition, self.slot, self.type_,
            self.parameters,
            permitted_string,
            self.default_value,
            value_string,
            connection_string
        )
    }
}

#[macro_export]
macro_rules! out_socket {
    (
        slot: $slot:expr,
        parameters: $params:expr,
        default: $default:expr
        $(, value: $value:expr)?
        $(, connection: $connection:expr)?
        $(, $($permitted:expr),* $(,)?)?
    ) => {{
        $crate::Socket::new(
            true,
            false,
            $slot,
            $crate::SocketType::Named,
            $params,
            $default,
            None $(.or(Some($value)))?,
            None $(.or(Some($connection)))?,
            &[$($($permitted),*)?]
        )
    }};
}

#[macro_export]
macro_rules! in_socket {
    (
        slot: $slot:expr,
        type: $type:expr,
        parameters: $params:expr,
        default: $default:expr
        $(, value: $value:expr)?
        $(, connection: $connection:expr)?
        $(, $($permitted:expr),* $(,)?)?
    ) => {{
        $crate::Socket::new(
            false,
            false,
            $slot,
            $type,
            $params,
            $default,
            None $(.or(Some($value)))?,
            None $(.or(Some($connection)))?,
            &[$($($permitted),*)?]
        )
    }};
}

#[macro_export]
macro_rules! rep_socket {
    (
        slot: $slot:expr,
        type: $type:expr,
        parameters: $params:expr,
        default: $default:expr
        $(, value: $value:expr)?
        $(, connection: $connection:expr)?
        $(, $($permitted:expr),* $(,)?)?
    ) => {{
        $crate::Socket::new(
            false,
            true,
            $slot,
            $type,
            $params,
            $default,
            None $(.or(Some($value)))?,
            None $(.or(Some($connection)))?,
            &[$($($permitted),*)?]
        )
    }};
}
