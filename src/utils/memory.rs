pub struct Arena<T> {
    items: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Arena { items: Vec::new() }
    }

    pub fn alloc(&mut self, item: T) -> &mut T {
        self.items.push(item);
        self.items.last_mut().unwrap()
    }
}
