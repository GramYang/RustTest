pub struct List<T> {
    head: Link<T>,
}

//这里把枚举Link替换成了Option
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            //take()恰恰就是mem::replace的封装！很巧妙！
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    //弹出当前元素并返回元素值
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            //map返回的是Option
            node.elem
        })
    }

    //返回head的值的不可变引用
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    //返回head的值的可变引用
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    //这里的&**node这么理解，&&**Box<Node<T>>等于Box<Node<T>>，而Box<Node<T>>等于&Node<T>
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

//实现迭代器，这个迭代器遍历一个元素后直接pop了
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

//迭代器返回T的不可变引用值
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

//迭代器返回T的可变引用值
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

//第二版链表，将mem::replace替换成了Option::take()，添加了pop，添加了调用pop的迭代器1，返回元素不可变和可变引用的迭代器2和3
//测试基本功能
pub fn test1(){
    let mut list = List::new();
    assert_eq!(list.pop(), None);
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));
    list.push(4);
    list.push(5);
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}

//测试新添加的peek
pub fn test2(){
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1); list.push(2); list.push(3);
    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));
    list.peek_mut().map(|value| {
        *value = 42
    });
    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
}

//测试into_iter，其next()会pop当前元素并返回
pub fn test3(){
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);
    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

//测试iter()，next()返回的是元素不可变引用，这里不会pop出元素
pub fn test4(){
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}

//上面的可变引用版本
pub fn test5(){
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);
    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
}