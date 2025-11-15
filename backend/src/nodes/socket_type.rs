use std::fmt::Display;

pub enum SocketType {
    Named,
    Number,
    Select,
    Switch,
    Text,
    Color
}

impl Display for SocketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketType::Named => write!(f, "named"),
            SocketType::Number => write!(f, "number"),
            SocketType::Select => write!(f, "select"),
            SocketType::Switch => write!(f, "switch"),
            SocketType::Text => write!(f, "text"),
            &SocketType::Color => write!(f, "colour")
        }
    }
}

#[macro_export]
macro_rules! socket_type {
    (named) => {
        $crate::SocketType::Named
    };

    (number) => {
        $crate::SocketType::Number
    };

    (select) => {
        $crate::SocketType::Select
    };

    (switch) => {
        $crate::SocketType::Switch
    };

    (text) => {
        $crate::SocketType::Text
    };

    (color) => {
        $crate::SocketType::Color
    };
}
