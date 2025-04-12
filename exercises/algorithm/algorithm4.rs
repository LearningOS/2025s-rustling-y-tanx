/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
    二分查找树
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>, // 左孩子节点
    right: Option<Box<TreeNode<T>>>,    // 右孩子节点
    // 这棵树是唯一所有权的，不需要多个指针指向同一个节点
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,     // 根节点
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        match self.root {
            Some(ref mut node) => node.insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match self.root {
            Some(ref node) => node.search(value),
            None => false,
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    // 对于重复的值只插入一次
    fn insert(&mut self, value: T) {
        if value == self.value {
            return;
        }

        //TODO
        if value < self.value {
            match self.left {
                None => self.left = Some(Box::new(TreeNode::new(value))),
                Some(ref mut left_node) => left_node.insert(value), // 这是 left_node: &mut Box<TreeNode<T>>
                // Box实现了DerefMut，因此left_node.insert就相当于*left_node.insert，它被自动解引用为&mut TreeNode<T>
                // 不使用Some(left_node)的原因 => 所有权发生了转移
            }
        }else {
            match self.right {
                None => self.right = Some(Box::new(TreeNode::new(value))),
                Some(ref mut right_node) => right_node.insert(value),
            }
        }
    }

    fn search(&self, value: T) -> bool {
        //TODO
        // 是否在BST中找到了这个值
        // base: 节点的值=value
        if self.value == value {
            return true;
        }

        // 在左、右子树中继续查找
        if value < self.value {
            // 在左子树中查找
            if let Some(ref left_node) = self.left {
                return left_node.search(value);
            }else
            {
                return false;
            }
        }else {
            // 在右子树中继续查找
            if let Some(ref right_node) = self.right {
                return right_node.search(value);
            }else {
                return false;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


