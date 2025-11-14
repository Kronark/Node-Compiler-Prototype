use std::fmt::Display;

pub enum SocketParameter {
    Named,
    Number {
        min: String,
        max: String,
        step: String,
    },
    Select,
    Switch,
    Text {
        min: String,
        max: String,
        valid: String,
    },
    Colour,
}

impl Display for SocketParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parameters:\n{}",
            "Display not implemented for SocketParameters."
        )
    }
}
