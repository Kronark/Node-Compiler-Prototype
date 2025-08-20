use std::ops::Deref;
use std::fmt::Display;

pub struct Type
{
    pub identifier: String
}

impl Type
{
    pub fn new(i : &str) -> Self
    {
        assert!(!i.is_empty());
        Self {
            identifier: String::from(i)
        }
    }
}

impl Deref for Type
{
    type Target = String;

    fn deref(&self) -> &Self::Target
    {
        &self.identifier
    }
}

impl Display for Type
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.identifier)
    }
}
