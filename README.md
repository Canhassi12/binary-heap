# binary-heap

## What is? 

A binary heap is a heap data structure that takes the form of a binary tree.
<img src="https://i.imgur.com/HNLQQ2A.png">

## Characteristics

- Shape property: a binary heap is a complete binary tree; that is, all levels of the tree, except possibly the last one (deepest) are fully filled, and, if the last level of the tree is not complete, the nodes of that level are filled from left to right.

- Heap property: the key stored in each node is either greater than or equal to (≥) or less than or equal to (≤) the keys in the node's children, according to some total order.

## Structure

Heaps are commonly implemented with an array. Any binary tree can be stored in an array, but because a binary heap is always a complete binary tree, it can be stored compactly.

Let n be the number of elements in the heap and i be an arbitrary valid index of the array storing the heap. If the tree root is at index 0, with valid indices 0 through n − 1, then each element a at index i has:

- Indice 2i + 1 to left children and 2i + 2 to the right children.
- Parent at index floor((i − 1) / 2).

Alternatively, if the tree root is at index 1, with valid indices 1 through n, then each element a at index i has

- Children at indices 2i and 2i + 1 respectively.
- Parent at index floor(i / 2).

<img src="https://i.imgur.com/Bmj8Tnh.png">

## Binary heap types

As you read above, binary heap have two differents types, the first is *`Min heap`*, when this is organized(heapify), the root of our binary heap must be the smallest value of heap.

The second one is *`Max heap`* when you heapify, the root it must be biggest value of heap.

## Where its used?

Binary heaps are also commonly employed in the heapsort sorting algorithm, which is an in-place algorithm because binary heaps can be implemented as an implicit data structure, storing keys in an array and using their relative positions within that array to represent child–parent relationships.

## My code

I did the *`Min heap`* in rust to practice, and in the code, we have a 'insert' function, to insert a new value in heap, 'delete_minimun' to delete the root of heap, and 'delete_element, to delete a specif element by index number' and the last one, 'heapify' function to organize and reach binary heap conditions.

## Reference 

[Binary heap - wikipedia](https://en.wikipedia.org/wiki/Binary_heap)
