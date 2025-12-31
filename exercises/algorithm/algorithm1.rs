/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

// 定义链表节点结构体，使用泛型 T 支持任意类型
#[derive(Debug)]
struct Node<T> {
    val: T,                         // 节点存储的值
    next: Option<NonNull<Node<T>>>, // 指向下一个节点的指针，使用 NonNull 保证非空
}

impl<T> Node<T> {
    // 创建一个新节点
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}

// 定义链表结构体，使用泛型 T 支持任意类型
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,                     // 链表长度
    start: Option<NonNull<Node<T>>>, // 链表头节点的指针
    end: Option<NonNull<Node<T>>>,   // 链表尾节点的指针
}

// 为 LinkedList 实现 Default trait，提供默认值
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    // 创建一个空链表
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    // 向链表末尾添加一个元素
    pub fn add(&mut self, obj: T) {
        // 在堆上创建一个新节点
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        // 将 Box 转换为裸指针，并包装在 NonNull 中
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            // 如果链表为空，新节点成为头节点
            None => self.start = node_ptr,
            // 如果链表不为空，将新节点添加到尾节点之后
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        // 更新尾节点指针
        self.end = node_ptr;
        // 增加链表长度
        self.length += 1;
    }

    // 获取指定索引位置的元素引用
    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    // 递归获取第 index 个节点的值
    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            // 如果节点为空，返回 None
            None => None,
            Some(next_ptr) => match index {
                // 如果索引为 0，返回当前节点的值
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                // 否则递归查找下一个节点，索引减 1
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
}

impl<T: Clone + PartialOrd> LinkedList<T> {
    // 合并两个有序链表，返回一个新的有序链表
    // TODO: 需要实现这个函数
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
        //TODO
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        let mut merged_list = Self::new();
        while i < list_a.length && j < list_b.length {
            if let Some(a) = list_a.get_ith_node(list_a.start, i as i32) {
                if let Some(b) = list_b.get_ith_node(list_b.start, j as i32) {
                    if a < b {
                        merged_list.add(a.clone());
                        i = i + 1;
                    } else {
                        merged_list.add(b.clone());
                        j = j + 1;
                    }
                }
            }
        }
        // 如果 list_a 还有剩余元素，添加到 merged_list
        while i < list_a.length {
            if let Some(a) = list_a.get_ith_node(list_a.start, i as i32) {
                merged_list.add(a.clone());
                i = i + 1;
            }
        }
        // 如果 list_b 还有剩余元素，添加到 merged_list
        while j < list_b.length {
            if let Some(b) = list_b.get_ith_node(list_b.start, j as i32) {
                merged_list.add(b.clone());
                j = j + 1;
            }
        }
        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

// 为 Node 实现 Display trait，支持格式化输出
impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            // 如果有下一个节点，输出当前值和下一个节点的值（递归）
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            // 如果是最后一个节点，只输出当前值
            None => write!(f, "{}", self.val),
        }
    }
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::LinkedList;

    // 测试创建数字链表
    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    // 测试创建字符串链表
    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    // 测试合并链表 - 第一个测试用例
    // 合并两个交替的有序链表：[1,3,5,7] 和 [2,4,6,8] -> [1,2,3,4,5,6,7,8]
    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        // 将 vec_a 的元素添加到 list_a
        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        // 将 vec_b 的元素添加到 list_b
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        // 合并两个链表
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        // 验证合并后的链表是否正确
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }

    // 测试合并链表 - 第二个测试用例
    // 合并两个长度不同的有序链表：[11,33,44,88,89,90,100] 和 [1,22,30,45] -> [1,11,22,30,33,44,45,88,89,90,100]
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        // 将 vec_a 的元素添加到 list_a
        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        // 将 vec_b 的元素添加到 list_b
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        // 合并两个链表
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        // 验证合并后的链表是否正确
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
