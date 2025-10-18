use std::ops::Deref;
use std::fmt::Display;
use std::hash::Hash;
use std::cmp::{PartialEq, Eq};

#[derive(Hash, PartialEq, Eq)]
pub struct DataType
{
    pub identifier: String
}

impl DataType
{
    pub fn new(i : &str) -> Self
    {
        assert!(!i.is_empty());
        Self {
            identifier: String::from(i)
        }
    }
}

impl Deref for DataType
{
    type Target = String;

    fn deref(&self) -> &Self::Target
    {
        &self.identifier
    }
}

impl Display for DataType
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
