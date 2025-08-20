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

    pub fn print(&self) { println!("{}", self.identifier.as_str()) }
}
