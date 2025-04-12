/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,  // 使用序列实现完全二叉堆
    comparator: fn(&T, &T) -> bool,     // 按照闭包指示的顺序来构造堆
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: Vec::new(),
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
        //TODO
        // 首先将该元素插入到最后
        self.items.push(value);
        self.count += 1;
        let mut idx = self.len() - 1;
        
        // 然后开始上浮，根据comparator依次交换父节点和子节点
        while idx > 0 {
            let mut parent = self.parent_idx(idx);
            if !(self.comparator)(&self.items[parent], &self.items[idx]) {
                // 不满足comparator，则开始交换
                self.items.swap(parent, idx);  
                idx = parent;
            }else {
                break;  // 如果不需要交换，说明这已经是它应该在的位置了
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        // 按照comparator指示的顺序弹出元素
        if self.is_empty() {
            return None;
        }
        // 首先将最后一个元素和堆顶元素进行交换
        self.items.swap(0, self.count - 1); 
        let result = self.items.pop();
        self.count -= 1;
        let len = self.len();

        // 然后将堆顶元素下推，在下推过程中就完成了更新堆的过程
        let mut idx = 0;
        loop {
            let left_idx = self.left_child_idx(idx);
            let right_idx = self.right_child_idx(idx);
            let mut best = idx;

            // 比较best和left_idx、right_idx之间的大小，判断是否可以交换
            if left_idx < len && !(self.comparator)(&self.items[best], &self.items[left_idx]) {
                best = left_idx;
            }

            if right_idx < len && !(self.comparator)(&self.items[best], &self.items[right_idx]) {
                best = right_idx;
            }

            if best != idx {
                // 更新
                self.items.swap(best, idx);
                idx = best;
            }else {
                break;  // 更新到了正确的位置
            }
        }

        result
    }

    fn parent_idx(&self, idx: usize) -> usize {
        (idx - 1) / 2
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
}

impl<T> Heap<T>
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

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        self.pop()
    }
}

/// 最小堆
pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

/// 最大堆
pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
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