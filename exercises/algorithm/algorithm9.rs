/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::fmt::Display;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + std::cmp::PartialOrd + std::fmt::Display,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        println!("count:{}",self.count);
        for i in 1..(self.len() + 1) {
            println!("{}",self.items[i]);
        }
        println!("-------------add");
        self.items.push(value);
        let mut idx = self.count + 1;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            if !((self.comparator)(&self.items[parent_idx] , &self.items[idx])) {
                self.items.swap(parent_idx, idx);
            }
            idx = parent_idx;
        }
        self.count = self.count + 1;
        //TODO
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		0
    }
}

impl<T:Display> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T:Copy  + std::fmt::Display> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        else{
            let t = self.items[1];
            println!("count:{}",self.count);
            for i in 1..(self.items.len()) {
                println!("{}",self.items[i]);
            }
            println!("-------------before");

            self.items.swap( 1 , self.count);
            self.items.pop();
            self.count -= 1;
            println!("count:{}",self.count);
            for i in 1..(self.items.len()) {
                println!("{}",self.items[i]);
            }
            println!("-------------after");
            let mut idx = 1;
            while true{
                let mut min = idx;
                if idx * 2 <= self.count && (self.comparator)(&self.items[idx * 2], &self.items[min]) {
                    min = idx * 2;
                }
                if idx * 2 + 1 <= self.count && (self.comparator)(&self.items[idx * 2 + 1], &self.items[min]) {
                    min = idx * 2 + 1;
                }

                if min != idx {
                    self.items.swap(min, idx);
                    idx = min;
                }
                else{
                    break;
                }
            }
            return Some(t);
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T:std::fmt::Display>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T:std::fmt::Display>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        println!("here");
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}