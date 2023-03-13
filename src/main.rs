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

        // pub fn delete() {

        // }
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

    println!("{:?}", a.arr);
}
