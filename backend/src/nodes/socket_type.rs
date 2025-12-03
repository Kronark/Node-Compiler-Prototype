use std::fmt::Display;

pub enum SocketType {
    Named,
    Number,
    Select,
    Switch,
    Text,
    Color,
}

impl Display for SocketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketType::Named => write!(f, "named"),
            SocketType::Number => write!(f, "number"),
            SocketType::Select => write!(f, "select"),
            SocketType::Switch => write!(f, "switch"),
            SocketType::Text => write!(f, "text"),
            &SocketType::Color => write!(f, "colour"),
        }
    }
}

#[macro_export]
macro_rules! socket_type {
    (named) => {
        $crate::nodes::socket_type::SocketType::Named
    };

    (number) => {
        $crate::nodes::socket_type::SocketType::Number
    };

    (select) => {
        $crate::nodes::socket_type::SocketType::Select
    };

    (switch) => {
        $crate::nodes::socket_type::SocketType::Switch
    };

    (text) => {
        $crate::nodes::socket_type::SocketType::Text
    };

    (color) => {
        $crate::nodes::socket_type::SocketType::Color
    };
}
