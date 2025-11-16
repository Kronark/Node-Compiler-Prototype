use std::collections::BTreeSet;

#[derive(Clone)]
pub struct Pool {
    allocated: BTreeSet<u32>,
}

impl Pool {
    pub fn new() -> Self {
        Self {
            allocated: BTreeSet::new(),
        }
    }

    pub fn allocate(&mut self) -> u32 {
        let mut id = 0;
        while self.allocated.contains(&id) {
            id += 1;
        }
        self.allocated.insert(id);
        id
    }

    pub fn reserve(&mut self, id: u32) -> u32 {
        self.allocated.insert(id);
        id
    }

    pub fn release(&mut self, id: u32) {
        self.allocated.remove(&id);
    }

    pub fn is_used(&self, id: u32) -> bool {
        self.allocated.contains(&id)
    }
}

#[macro_export]
macro_rules! pool {
    () => {
        $crate::nodes::pool::Pool::new()
    };
}
