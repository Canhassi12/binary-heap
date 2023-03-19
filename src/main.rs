fn main() {
    struct MinHeap {
        arr: Vec<i32>,
        size: usize,
        capacity: usize,
    }

    impl MinHeap {
        pub fn insert(&mut self, value: i32) {
            if self.size == self.capacity {
                eprintln!("Cannot insert {}. Heap is already full!", value);
                return;
            }

            self.size += 1;
            self.arr.push(value);

            let mut index = self.size - 1;

            while index > 0 && self.arr[(index - 1) / 2] > self.arr[index] {
                self.arr.swap(index, (index - 1) / 2);
                index = (index - 1) / 2;
            }
        }

        pub fn delete_minimum(&mut self) {
            if self.arr.is_empty() || self.size == 0 {
                eprintln!("Heap not found, or size its zero");
            }

            let size = self.size;
            let last_element = self.arr[size - 1];

            self.arr[0] = last_element;

            self.size -= 1;

            self.heapify(0);

            self.arr.remove(self.size);
        }

        fn heapify(&mut self, index: usize) {
            if self.size <= 1 {
                return
            } 

            let left_child = 2*index + 1;

            let right_child = 2*index + 2;

            let mut smallest = index;
 
            if left_child < self.size && self.arr[left_child] < self.arr[index] {
                smallest = left_child;
            }

            if right_child < self.size && self.arr[right_child] < self.arr[index] {
                smallest = right_child;
            }

            if smallest != index {
                let temp = self.arr[index];
                
                self.arr[index] = self.arr[smallest];

                self.arr[smallest] = temp;

                self.heapify(smallest);
            }
        }

        pub fn delete_element(&mut self, index: usize) {
            self.arr[index] = std::i32::MIN;

            let mut current = index;

            while current > 0 && self.arr[(current - 1) / 2] > self.arr[current] {
                let parent = (current - 1) / 2;

                self.arr.swap(parent, current);

                current = parent;
            
            }
            self.delete_minimum();
        }
    }

    let mut a = MinHeap {
        arr: Vec::with_capacity(10),
        size: 0,
        capacity: 10,
    };
        

    a.insert(12);
    a.insert(3);
    a.insert(42);
    a.insert(15);
    a.insert(22);
    a.insert(80);
    a.insert(16);
    a.insert(2);

    println!("{:?}", a.arr);

    a.delete_minimum();

    println!("{:?}", a.arr);

    a.delete_element(0);

    println!("{:?}", a.arr);
}
