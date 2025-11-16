use crate::nodes::connection::Connection;
use crate::nodes::data::Data;
use crate::nodes::data_type::DataType;
use crate::nodes::socket_parameters::SocketParameters;
use crate::nodes::socket_type::SocketType;
use std::fmt::Display;
use std::collections::HashSet;

pub struct Socket {
    pub is_outgoing: bool,
    pub is_repetition: bool,
    pub type_: SocketType,
    pub parameters: SocketParameters,
    permitted: HashSet<DataType>,
    pub default: Data,
    pub actual: Option<Data>,
    pub connection: Option<Connection>,
}

impl Socket {
    pub fn new(
        io: bool,
        ir: bool,
        t: SocketType,
        param: SocketParameters,
        d: Data,
        a: Option<Data>,
        c: Option<Connection>,
        perm: impl IntoIterator<Item = DataType>,
    ) -> Self {
        let permitted_set = perm.into_iter().collect::<HashSet<_>>();

        Self {
            is_outgoing: io,
            is_repetition: ir,
            type_: t,
            parameters: param,
            permitted: permitted_set,
            default: d,
            actual: a,
            connection: c,
        }
    }

    pub fn permit(&mut self, data_type: DataType) {
        self.permitted.insert(data_type);
    }

    pub fn forbid(&mut self, data_type: &DataType) {
        self.permitted.remove(data_type);
    }

    pub fn is_permitted(&self, query: DataType) -> bool {
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
            let bullet_points = self
                .permitted
                .iter()
                .map(|dt| format!("• {}", dt))
                .collect::<Vec<_>>()
                .join("\n");
            format!("permitted:\n{}", indent(&bullet_points, 1))
        };

        writeln!(
            f,
            "{} {} {}\n{}\n{}\n{}\n{}",
            direction,
            repetition,
            self.type_,
            indent(&self.parameters.to_string(), 1),
            indent(&permitted_string, 1),
            indent(&format!("default:"), 1),
            indent(&format!("{}", self.default), 2),
        )?;

        let actual_string = match &self.actual {
            Some(actual) => format!(
                "{}\n{}",
                indent("actual:", 1),
                indent(&format!("{}", actual), 2)
            ),
            None => indent("no actual data", 1).to_owned(),
        };

        writeln!(f, "{}", actual_string,)?;

        let connection_string = match &self.connection {
            Some(connection) => format!(
                "{}\n{}",
                indent("connection:", 1),
                indent(&format!("{}", connection), 2)
            ),
            None => indent("no connection data", 1).to_owned(),
        };

        writeln!(f, "{}", connection_string,)
    }
}

#[macro_export]
macro_rules! out_socket {
    (
        default: $default:expr
        $(, actual: $actual:expr)?
        $(, connection: $connection:expr)?
        $(, permitted: [ $($permitted:expr),* $(,)? ])?
    ) => {{
        $crate::nodes::socket::Socket::new(
            true,
            false,
            $crate::nodes::socket_type::SocketType::Named,
            $crate::nodes::socket_parameters::SocketParameters::Named,
            $default,
            None $(.or(Some($actual)))?,
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
        $(, actual: $actual:expr)?
        $(, connection: $connection:expr)?
        $(, permitted: [ $($permitted:expr),* $(,)? ])?
    ) => {{
        $crate::nodes::socket::Socket::new(
            false,
            false,
            $type,
            $params,
            $default,
            None $(.or(Some($actual)))?,
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
        $(, actual: $actual:expr)?
        $(, connection: $connection:expr)?
        $(, permitted: [ $($permitted:expr),* $(,)? ])?
    ) => {{
        $crate::nodes::socket::Socket::new(
            false,
            true,
            $type,
            $params,
            $default,
            None $(.or(Some($actual)))?,
            None $(.or(Some($connection)))?,
            (&[$($($permitted),*)?]).iter().cloned()
        )
    }};
}
