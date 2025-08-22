use std::fmt::Display;

// TODO: might be smart to move the "default value" parameter to the main socket struct, as all socket types have it in common
enum SocketParameter {
    Named(String),
    Number {
        min: String,
        max: String,
        step: String,
        default: String,
    },
    Select(String),
    Switch {
        active: String,
        inactive: String,
        default: String,
    },
    Text {
        min: String,
        max: String,
        valid: String,
        default: String,
    },
}

impl Display for SocketParameter
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "{}",
            "Display not implemented for SocketParameters."
        )
    }
}
