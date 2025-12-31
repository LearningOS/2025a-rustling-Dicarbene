/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


// 二叉树节点结构体
#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,                          // 节点存储的值
    left: Option<Box<TreeNode<T>>>,    // 左子节点（可选，使用 Box 进行堆分配）
    right: Option<Box<TreeNode<T>>>,   // 右子节点（可选，使用 Box 进行堆分配）
}

// 二叉搜索树结构体
#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,    // 根节点（可选，空树时为 None）
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // 创建一个新的树节点
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,      // 初始时没有左子节点
            right: None,     // 初始时没有右子节点
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    // 创建一个空的二叉搜索树
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // 向 BST 中插入一个值
    fn insert(&mut self, value: T) {
        //TODO
        if let None = self.root {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }
        let mut current = self.root.as_mut().unwrap();
        loop {
            match value.cmp(&current.value) {
                Ordering::Less => {
                    if let None = current.left {
                        current.left = Some(Box::new(TreeNode::new(value)));
                        break;
                    }
                    current = current.left.as_mut().unwrap();
                }
                Ordering::Greater => {
                    if let None = current.right {
                        current.right = Some(Box::new(TreeNode::new(value)));
                        break;
                    }
                    current = current.right.as_mut().unwrap();
                }
                Ordering::Equal => {
                    break;
                }
            }
        }
    }

    // 在 BST 中搜索一个值
    fn search(&self, value: T) -> bool {
        //TODO
        if let None = self.root {
            return false;
        }
        let mut current = self.root.as_ref().unwrap();
        loop {
            match value.cmp(&current.value) {
                Ordering::Less => {
                    if let None = current.left {
                        return false;
                    }
                    current = current.left.as_ref().unwrap();
                }
                Ordering::Greater => {
                    if let None = current.right {
                        return false;
                    }
                    current = current.right.as_ref().unwrap();
                }
                Ordering::Equal => {
                    return true;
                }
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // 向树中插入一个节点
    fn insert(&mut self, value: T) {
        //TODO
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


