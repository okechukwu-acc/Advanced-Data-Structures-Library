#[derive(Debug, Clone)]
pub struct BinaryHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> BinaryHeap<T> {
    pub fn new() -> Self {
        BinaryHeap { data: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.heapify_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.len() > 1 {
            self.data.swap(0, self.data.len() - 1);
            let item = self.data.pop();
            self.heapify_down(0);
            item
        } else {
            self.data.pop()
        }
    }

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.data[index] > self.data[parent] {
                self.data.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        let len = self.data.len();
        let mut largest = index;

        loop {
            let left = 2 * index + 1;
            let right = 2 * index + 2;

            if left < len && self.data[left] > self.data[largest] {
                largest = left;
            }

            if right < len && self.data[right] > self.data[largest] {
                largest = right;
            }

            if largest != index {
                self.data.swap(index, largest);
                index = largest;
            } else {
                break;
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
