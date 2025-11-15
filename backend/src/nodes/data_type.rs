use std::{fmt::Display};
use crate::{make_interner, nodes::identifier::Identifier};

// ========== Data Type Interner ==========

make_interner!(DATA_TYPE_INTERNER, DataTypeInterner, DataType, data_type_interner);

// ========== Data Type ==========

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct DataType {
    is_package: bool,
    identifier: Identifier
}

impl DataType {
    pub fn new(i : Identifier, p : bool) -> Arc<Self> {
        data_type_interner().intern(Self {
            is_package: p,
            identifier: i
        })
    }

    pub fn is_package(&self) -> bool {
        self.is_package
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut packaged = "singular";
        if self.is_package {
            packaged = "packaged";
        }
        
        write!(
            f,
            "{} ~ {}",
            packaged, self.identifier
        )
    }
}

#[macro_export]
macro_rules! data_type {
    ($identifier:expr) => {
        $crate::DataType::new($identifier, false)
    };
    ($identifier:expr, $is_package:expr) => {{
        $crate::DataType::new($identifier, $is_package)
    }};
}
