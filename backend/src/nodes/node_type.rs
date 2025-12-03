use crate::nodes::identifier::Identifier;
use std::fmt::Display;

// ========== Node Type ==========

#[derive(Hash, PartialEq, Eq)]
pub struct NodeType {
    identifier: Identifier,
}

impl NodeType {
    pub fn new(i: Identifier) -> Self {
        Self { identifier: i }
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

#[macro_export]
macro_rules! node_type {
    ($identifier:expr) => {{ $crate::nodes::node_type::NodeType::new($identifier) }};
}
