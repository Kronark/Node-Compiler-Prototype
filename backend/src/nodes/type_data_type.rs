use std::ops::Deref;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, Eq};

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
        write!(
            f,
            "{}",
            self.identifier
        )
    }
}

impl Hash for Type
{
    fn hash<H : Hasher>(&self, state: &mut H)
    {
        self.identifier.hash(state)
    }
}

impl PartialEq for Type
{
    fn eq(&self, other: &Self) -> bool
    {
        self.identifier == other.identifier
    }
}

impl Eq for Type {}
