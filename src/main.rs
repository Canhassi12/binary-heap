struct MinHeap {
    arr: Vec<i32>,
    size: usize,
    capacity: usize,
}

impl MinHeap {
    pub fn insert(&mut self, value: i32) -> Result<(), String> {
        if self.size == self.capacity {
            return Err(String::from("Cannot insert more values. Heap is already full!"));
        }

        self.size += 1;
        self.arr.push(value);

        let mut index = self.size - 1;

        while index > 0 && self.arr[(index - 1) / 2] > self.arr[index] {
            self.arr.swap(index, (index - 1) / 2);
            index = (index - 1) / 2;
        }

        Ok(())
    }

    pub fn delete_minimum(&mut self) -> Result<(), String> {
        if self.arr.is_empty() || self.size == 0 {
            return Err(String::from("Heap not found, or size its zero"));
        }

        let size = self.size;
        let last_element = self.arr[size - 1];

        self.arr[0] = last_element;

        self.size -= 1;

        self.heapify(0);

        self.arr.remove(self.size);

        Ok(())
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
        self.delete_minimum().unwrap();
    }
}

fn main() {
    let mut a = MinHeap {
        arr: Vec::with_capacity(10),
        size: 0,
        capacity: 10,
    };
        
    a.insert(12).unwrap();
    a.insert(3).unwrap();
    a.insert(42).unwrap();
    a.insert(15).unwrap();
    a.insert(22).unwrap();
    a.insert(80).unwrap();
    a.insert(16).unwrap();
    a.insert(2).unwrap();

    println!("{:?}", a.arr);

    a.delete_minimum().unwrap();

    println!("{:?}", a.arr);

    a.delete_element(0);

    println!("{:?}", a.arr);
}

mod tests {
    use super::*;

    fn _heap_provider() -> MinHeap {
        let heap = MinHeap {
            arr: Vec::with_capacity(10),
            size: 0,
            capacity: 10,
        };
        
        heap
    }

    fn _heap_with_values() -> MinHeap{
        let mut heap = _heap_provider();

        heap.insert(12).unwrap();
        heap.insert(3).unwrap();
        heap.insert(42).unwrap();
        heap.insert(15).unwrap();
        heap.insert(22).unwrap();
        heap.insert(80).unwrap();
        heap.insert(16).unwrap();
        heap.insert(2).unwrap();

        heap
    }

    #[test]
    fn it_can_insert_some_values_in_heap() {
        let mut heap = _heap_provider();

        heap.insert(12).unwrap();
        heap.insert(3).unwrap();
        heap.insert(42).unwrap();
        heap.insert(15).unwrap();
        heap.insert(22).unwrap();
        heap.insert(80).unwrap();
        heap.insert(16).unwrap();
        heap.insert(2).unwrap();

        assert_eq!(heap.arr, [2, 3, 16, 12, 22, 80, 42, 15]);
    }

    #[test]
    fn it_can_delete_the_root_element() {
        let mut heap = _heap_with_values();

        heap.delete_minimum().unwrap();

        assert_eq!(heap.arr, [3, 12, 16, 15, 22, 80, 42]);
    }

    #[test]
    fn it_can_delete_specific_element_by_index() {
        let mut heap = _heap_with_values();

        heap.delete_element(0);

        assert_eq!(heap.arr, [3, 12, 16, 15, 22, 80, 42]);
    }

    #[test]
    fn it_should_not_insert_more_values_with_the_smaller_capacity() {
        let mut heap = _heap_with_values();

        heap.size = heap.capacity;
        let result = heap.insert(94);

        assert_eq!(result, Err(String::from("Cannot insert more values. Heap is already full!")));
    }

    #[test]
    fn it_can_delete_the_root_with_empty_heap_or_size_0() {
        let mut heap = _heap_provider();

        heap.size = 0;

        let result = heap.delete_minimum();

        assert_eq!(result, Err(String::from("Heap not found, or size its zero")));
    }
}
