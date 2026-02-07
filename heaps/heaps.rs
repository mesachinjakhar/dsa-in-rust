struct MinHeap {
    data: Vec<i32>
}

impl MinHeap {
    fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        self.data.push(val);
        self.heapify_up(self.data.len() - 1); 
    } 

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = {index - 1}/2; 
            if self.data[index] < self.data[parent] {
                self.data.swap(index, parent);
                index = parent;
            }
            else {
                break;
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }

        let min = self.data[0];
        let last = self.data.pop().unwrap();

        if !self.data.is_empty() {
            self.data[0] = last; 
            self.heapify_down(0); 
        }

        Some(min)
    }

    fn heapify_down(&mut self, mut index: usize) {
        let len = self.data.len(); 

        loop {
            let left = 2 * index + 1; 
            let right = 2 * index + 2; 
            let mut smallest = index; 

            if left < len && self.data[left] < self.data[smallest] {
                smallest = left;
            }

            if right < len && self.data[right] < self.data[smallest] {
                smallest = right;
            }

            if smallest == index {
                break;
            }

            self.data.swap(index, smallest);
            index = smallest;
        }

    }
}


fn create_heap() {
    let mut heap = MinHeap::new(); 

    heap.push(5);
    heap.push(3);
    heap.push(8);
    heap.push(1);

    print!("{:?}", heap.pop());
    println!("{:?}", heap.pop())
}