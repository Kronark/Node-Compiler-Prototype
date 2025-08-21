use std::fmt::Display;

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
