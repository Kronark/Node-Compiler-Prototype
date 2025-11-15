use std::fmt::Display;

pub enum SocketParameters {
    Named,
    Number {
        min: String,
        max: String,
        step: String
    },
    Select {
        options: Vec<String>
    },
    Switch,
    Text {
        min: String,
        max: String,
        valid: String
    },
    Color
}

impl Display for SocketParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketParameters::Named
            | SocketParameters::Switch
            | SocketParameters::Color => {
                write!(f, "no parameters")
            }

            SocketParameters::Select { options } => {
                if options.is_empty() {
                    return write!(f, "no options");
                }

                writeln!(f, "options:")?;
                for opt in options {
                    writeln!(f, "    • {}", opt)?;
                }

                Ok(())
            }

            SocketParameters::Number { min, max, step} => {
                writeln!(f, "parameters:")?;
                writeln!(f, "    • minimum: {}", min)?;
                writeln!(f, "    • maximum: {}", max)?;
                write!(f, "    • step: {}", step)
            }

            SocketParameters::Text { min, max, valid } => {
                let valid_text = if valid.is_empty() {
                    "<all>"
                } else {
                    valid.as_str()
                };

                writeln!(f, "parameters:")?;
                writeln!(f, "    • minimum: {}", min)?;
                writeln!(f, "    • maximum: {}", max)?;
                write!(f,   "    • valid: {}", valid_text)
            }
        }
    }
}

#[macro_export]
macro_rules! socket_parameters {
    (named) => {
        $crate::SocketParameters::Named
    };

    (switch) => {
        $crate::SocketParameters::Switch
    };

    (color) => {
        $crate::SocketParameters::Color
    };

    (number, min: $min:expr, max: $max:expr, step: $step:expr) => {
        $crate::SocketParameters::Number {
            min: $min.into(),
            max: $max.into(),
            step: $step.into(),
        }
    };

    (text, min: $min:expr, max: $max:expr, valid: $valid:expr) => {
        $crate::SocketParameters::Text {
            min: $min.into(),
            max: $max.into(),
            valid: $valid.into(),
        }
    };

    (select $(, $opt:expr )* $(,)?) => {
        $crate::SocketParameters::Select {
            options: vec![$($opt.into(),)*]
        }
    };
}
