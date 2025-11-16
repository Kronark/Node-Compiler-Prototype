use std::collections::BTreeSet;

#[derive(Clone)]
pub struct Pool {
    free: BTreeSet<u32>,
    next: u32,
}

impl Pool {
    pub fn new() -> Self {
        Self {
            free: BTreeSet::new(),
            next: 0,
        }
    }

    pub fn allocate(&mut self) -> u32 {
        if let Some(&id) = self.free.iter().next() {
            self.free.remove(&id);
            id
        } else {
            let id = self.next;
            self.next += 1;
            id
        }
    }

    pub fn release(&mut self, id: u32) {
        if id + 1 == self.next {
            self.next = self.next.saturating_sub(1);

            while self.free.contains(&(self.next - 1)) {
                self.next -= 1;
                self.free.remove(&self.next);
            }
        } else if id < self.next {
            self.free.insert(id);
        }
    }

    pub fn is_used(&self, id: u32) -> bool {
        !self.free.contains(&id)
    }
}

#[macro_export]
macro_rules! pool {
    () => {
        $crate::nodes::pool::Pool::new()
    };
}
