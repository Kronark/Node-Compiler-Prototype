use std::{fmt::Display};
use crate::{make_interner, nodes::identifier::Identifier};

// ========== Node Settings Interner ==========

make_interner!(NODE_TYPE_INTERNER, NodeTypeInterner, NodeType, node_type_interner);

// ========== Data Type ==========

#[derive(Hash, PartialEq, Eq)]
pub struct NodeType {
    identifier: Identifier
}

impl NodeType {
    pub fn new(i : Identifier) -> Arc<Self> {
        node_type_interner().intern(Self {
            identifier: i
        })
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.identifier
        )
    }
}

#[macro_export]
macro_rules! node_type {
    ($identifier:expr) => {{
        $crate::NodeType::new($identifier)
    }};
}
