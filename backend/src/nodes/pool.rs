use std::collections::BTreeSet;

#[derive(Clone)]
pub struct Pool {
    allocated: BTreeSet<u32>,
    lowest: u32,
}

impl Pool {
    pub fn new() -> Self {
        Self {
            allocated: BTreeSet::new(),
            lowest: 0,
        }
    }

    pub fn allocate(&mut self) -> u32 {
        let mut id = self.lowest;
        while self.allocated.contains(&id) {
            id += 1;
        }
        self.allocated.insert(id);
        self.cache_lowest(id, true);
        id
    }

    pub fn reserve(&mut self, id: u32) -> u32 {
        self.allocated.insert(id);
        self.cache_lowest(id, true);
        id
    }

    pub fn release(&mut self, id: u32) {
        self.allocated.remove(&id);
        self.cache_lowest(id, false);
    }

    pub fn is_used(&self, id: u32) -> bool {
        self.allocated.contains(&id)
    }

    fn cache_lowest(&mut self, id: u32, is_allocated: bool) {
        if is_allocated {
            self.lowest = id + 1;
        } else if id < self.lowest {
            self.lowest = id;
        }
    }
}

#[macro_export]
macro_rules! pool {
    () => {
        $crate::nodes::pool::Pool::new()
    };
}
