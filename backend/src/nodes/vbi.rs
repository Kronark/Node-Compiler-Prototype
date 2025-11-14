use std::fmt::Display;

pub struct VBI(pub u32);

impl Display for VBI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(
            f,
            "{}",
            self.0
        )
    }
}
