use std::{collections::HashSet, fmt::Display, sync::{Arc, OnceLock}};
use parking_lot::{RwLock};
use crate::{nodes::identifier::{Identifier}};

// ========== Globals ==========

static TYPE_INTERNER: OnceLock<TypeInterner> = OnceLock::new();

// ========== Data Type ==========

#[derive(Hash, PartialEq, Eq)]
pub struct DataType {
    is_package: bool,
    identifier: Identifier
}

impl DataType {
    pub fn new(i : Identifier, p : bool) -> Self {
        Self {
            is_package: p,
            identifier: i
        }
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

// ========== Data Type Interner ==========

pub struct TypeInterner {
    data: RwLock<HashSet<Arc<DataType>>>
}

impl TypeInterner {
    fn new() -> Self {
        Self {
            data: RwLock::new(HashSet::new())
        }
    }

    fn intern(&self, datum: DataType) -> Arc<DataType> {
        let mut data = self.data.write();

        if let Some(existing_datum) = data.get(&datum) {
            return existing_datum.clone();
        }

        let new_reference = Arc::new(datum);
        data.insert(new_reference.clone());
        new_reference
    }
}

pub fn type_interner() -> &'static TypeInterner {
    TYPE_INTERNER.get_or_init(|| TypeInterner::new())
}
