use std::fmt::Display;
use std::{collections::HashSet, sync::Arc};
use crate::nodes::connection::Connection;
use crate::nodes::socket_type::SocketType;
use crate::nodes::socket_parameters::SocketParameters;
use crate::nodes::data_value::DataValue;
use crate::nodes::data_type::DataType;

// FIXME: socket currently only storing default and active values, no data types for either. make a data object that combines dataValue and dataType
// FIXME: overhaul socket slot as separate object with node space based assignment
pub struct Socket {
    pub is_outgoing: bool,
    pub is_repetition: bool,
    pub type_: SocketType,
    pub parameters: SocketParameters,
    permitted: HashSet<Arc<DataType>>,
    pub default_value: DataValue,
    pub actual: Option<DataValue>,
    pub connection: Option<Connection>
}

impl Socket {
    pub fn new(
        io: bool,
        ir: bool,
        t: SocketType,
        param: SocketParameters,
        d: DataValue,
        a: Option<DataValue>,
        c: Option<Connection>,
        perm: impl IntoIterator<Item = Arc<DataType>>,
    ) -> Self {
        let permitted_set = perm.into_iter().collect::<HashSet<_>>();

        Self {
            is_outgoing: io,
            is_repetition: ir,
            type_: t,
            parameters: param,
            permitted: permitted_set,
            default_value: d,
            actual: a,
            connection: c
        }
    }

    pub fn permit(&mut self, data_type: Arc<DataType>) {
        self.permitted.insert(data_type);
    }

    pub fn forbid(&mut self, data_type: &Arc<DataType>) {
        self.permitted.remove(data_type);
    }

    pub fn is_permitted(&self, query : DataType) -> bool {
        self.permitted.contains(&query)
    }
}

impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn indent(text: &str, level: usize) -> String {
            let pad = "    ".repeat(level);
            text.lines()
                .map(|line| format!("{}{}", pad, line))
                .collect::<Vec<_>>()
                .join("\n")
        }

        let direction: &str = if self.is_outgoing { "<" } else { ">" };
        let repetition: &str = if self.is_repetition { "↻" } else { "⁠—" };

        let permitted_string = if self.permitted.is_empty() {
            "no permitted types".to_string()
        } else {
            let bullet_points = self.permitted
                .iter()
                .map(|dt| format!("• {}", dt))
                .collect::<Vec<_>>()
                .join("\n");
            format!("permitted:\n{}", indent(&bullet_points, 1))
        };

        let value_string = match &self.actual {
            Some(value) => value.to_string(),
            None => "no actual data".to_owned(),
        };

        let connection_string = match &self.connection {
            Some(conn) => conn.to_string(),
            None => "no connection".to_owned(),
        };

        write!(
            f,
            "{} {} {}\n{}\n{}\n{}\n{}\n{}",
            direction, repetition, self.type_,
            indent(&self.parameters.to_string(), 1),
            indent(&permitted_string, 1),
            indent(&format!("default: {}", self.default_value), 1),
            indent(&format!("actual: {}", value_string), 1),
            indent(&format!("connection: {}", connection_string), 1),
        )
    }
}

#[macro_export]
macro_rules! out_socket {
    (
        default: $default:expr
        $(, value: $value:expr)?
        $(, connection: $connection:expr)?
        $(, $($permitted:expr),* $(,)?)?
    ) => {{
        $crate::nodes::socket::Socket::new(
            true,
            false,
            $crate::nodes::socket_type::SocketType::Named,
            $crate::nodes::socket_parameters::SocketParameters::Named,
            $default,
            None $(.or(Some($value)))?,
            None $(.or(Some($connection)))?,
            (&[$($($permitted),*)?]).iter().cloned()
        )
    }};
}

#[macro_export]
macro_rules! in_socket {
    (
        type: $type:expr,
        parameters: $params:expr,
        default: $default:expr
        $(, value: $value:expr)?
        $(, connection: $connection:expr)?
        $(, $($permitted:expr),* $(,)?)?
    ) => {{
        $crate::nodes::socket::Socket::new(
            false,
            false,
            $type,
            $params,
            $default,
            None $(.or(Some($value)))?,
            None $(.or(Some($connection)))?,
            (&[$($($permitted),*)?]).iter().cloned()
        )
    }};
}

#[macro_export]
macro_rules! rep_socket {
    (
        type: $type:expr,
        parameters: $params:expr,
        default: $default:expr
        $(, value: $value:expr)?
        $(, connection: $connection:expr)?
        $(, $($permitted:expr),* $(,)?)?
    ) => {{
        $crate::nodes::socket::Socket::new(
            false,
            true,
            $type,
            $params,
            $default,
            None $(.or(Some($value)))?,
            None $(.or(Some($connection)))?,
            (&[$($($permitted),*)?]).iter().cloned()
        )
    }};
}
