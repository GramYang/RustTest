use std::ptr;

//将tail修改成了Node可变裸指针
pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

//将Rc+RefCell替换回了Box，同第二版
type Link<T> = Option<Box<Node<T>>>;

//删除了prev
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    //tail初始化为可变空指针
    pub fn new() -> Self {
        List { head: None, tail: ptr::null_mut() }
    }

    //向链表尾部推入元素
    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem,
            next: None,
        });
        let raw_tail: *mut _ = &mut *new_tail;
        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        //更新tail
        self.tail = raw_tail;
    }

    //从链表头部弹出元素并返回
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            head.elem
        })
    }

    //返回链表头部节点值的可变引用值
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    //返回链表头部节点值的不可变引用值
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    //同第二版
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    //同第二版
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

//第五版链表又变回单向链表，迭代器和Link改回了第二版，List在第二版的基础上使用了裸指针
//测试基本功能
pub fn test1(){
    let mut list = List::new();
    assert_eq!(list.pop(), None);
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(2));
    list.push(4);
    list.push(5);
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), None);
    list.push(6);
    list.push(7);
    assert_eq!(list.pop(), Some(6));
    assert_eq!(list.pop(), Some(7));
    assert_eq!(list.pop(), None);
}

//测试into_iter
pub fn test2(){
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);
    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);
}

//测试iter
pub fn test3(){
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

//测试iter_mut
pub fn test4(){
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);
    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), None);
}