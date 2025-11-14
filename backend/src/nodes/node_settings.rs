use std::{fmt::Display};
use crate::{make_interner, nodes::identifier::Identifier};

// ========== Node Settings Interner ==========

make_interner!(NODE_SETTINGS_INTERNER, NodeSettingsInterner, NodeSettings, node_settings_interner);

// ========== Data Type ==========

#[derive(Hash, PartialEq, Eq)]
pub struct NodeSettings {
    identifier: Identifier
}

impl NodeSettings {
    pub fn new(i : Identifier) -> Arc<Self> {
        node_settings_interner().intern(Self {
            identifier: i
        })
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

impl Display for NodeSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.identifier
        )
    }
}

#[macro_export]
macro_rules! node_settings {
    ($identifier:expr) => {{
        $crate::NodeSettings::new($identifier)
    }};
}
