/*
    double linked list reverse
    This problem requires you to reverse a doubly linked list
*/
// 双向链表反转问题
// 这个问题要求你反转一个双向链表

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

// 定义双向链表节点结构体，使用泛型 T 支持任意类型
#[derive(Debug)]
struct Node<T> {
    val: T,                         // 节点存储的值
    next: Option<NonNull<Node<T>>>, // 指向下一个节点的指针
    prev: Option<NonNull<Node<T>>>, // 指向前一个节点的指针
}

impl<T> Node<T> {
    // 创建一个新节点
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}

// 定义双向链表结构体，使用泛型 T 支持任意类型
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
        // 设置新节点的前驱指针为当前尾节点
        node.prev = self.end;
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

    // 反转双向链表
    // TODO: 需要实现这个函数
    pub fn reverse(&mut self) {
        // TODO: 实现链表反转逻辑
        // 1. 交换头节点和尾节点指针
        // 2. 遍历链表，交换每个节点的 next 和 prev 指针
        // 3. 更新头节点和尾节点指针
        let mut current = self.start;
        while let Some(node) = current {
            let next = unsafe { (*node.as_ptr()).next };
            let prev = unsafe { (*node.as_ptr()).prev };
            unsafe {
                (*node.as_ptr()).next = prev;
                (*node.as_ptr()).prev = next;
            }
            current = next;
        }
        self.start = self.end;
        self.end = current;
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

    // 测试反转链表 - 第一个测试用例
    // 反转链表 [2,3,5,11,9,7] -> [7,9,11,5,3,2]
    #[test]
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3, 5, 11, 9, 7];
        let reverse_vec = vec![7, 9, 11, 5, 3, 2];
        // 将 original_vec 的元素添加到链表
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        // 反转链表
        list.reverse();
        println!("Reversed Linked List is {}", list);
        // 验证反转后的链表是否正确
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    // 测试反转链表 - 第二个测试用例
    // 反转链表 [34,56,78,25,90,10,19,34,21,45] -> [45,21,34,19,10,90,25,78,56,34]
    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let reverse_vec = vec![45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
        // 将 original_vec 的元素添加到链表
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        // 反转链表
        list.reverse();
        println!("Reversed Linked List is {}", list);
        // 验证反转后的链表是否正确
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }
}