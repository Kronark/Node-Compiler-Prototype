#[macro_export]
macro_rules! make_interner {
    ($global_name:ident, $interner_name:ident, $type_name:ty, $getter_name:ident) => {
        use std::{collections::HashSet, sync::{Arc, OnceLock}};
        use parking_lot::RwLock;

        static $global_name: OnceLock<$interner_name> = OnceLock::new();

        pub struct $interner_name {
            data: RwLock<HashSet<Arc<$type_name>>>
        }

        impl $interner_name {
            fn new() -> Self {
                Self {
                    data: RwLock::new(HashSet::new())
                }
            }

            pub fn intern(&self, datum: $type_name) -> Arc<$type_name> {
                let mut data = self.data.write();

                if let Some(existing_datum) = data.get(&datum) {
                    return existing_datum.clone();
                }

                let new_reference = Arc::new(datum);
                data.insert(new_reference.clone());
                new_reference
            }
        }

        pub fn $getter_name() -> &'static $interner_name {
            $global_name.get_or_init(|| $interner_name::new())
        }
    };
}
