use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

//用Rc替换了Box，如：
// list1 = A -> B -> C -> D
// list2 = tail(list1) = B -> C -> D
// list3 = push(list2, X) = X -> B -> C -> D
//这个例子里面，B的所有权会被共享，因此需要用Rc
type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    //使用append替代了push
    pub fn append(&self, elem: T) -> List<T> {
        List { head: Some(Rc::new(Node {
            elem,
            next: self.head.clone(),
        }))}
    }

    //用tail替代了pop，因为这里返回的不是Option，所以不能用map，而是用and_then
    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

//因为用Rc替换了Box，所以原来的cur_link = boxed_node.next.take();需要修改。
//take()会修改里面的值，而Rc里的值可能被共享，因此这里需要使用try_unwrap来只取值不改变值。
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
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

//第三版链表，使用Rc替换了Box，替换了push和pop
//测试基本功能
pub fn test1(){
    let list = List::new();
    assert_eq!(list.head(), None);
    let list = list.append(1).append(2).append(3);
    assert_eq!(list.head(), Some(&3));
    let list = list.tail();
    assert_eq!(list.head(), Some(&2));
    let list = list.tail();
    assert_eq!(list.head(), Some(&1));
    let list = list.tail();
    assert_eq!(list.head(), None);
    let list = list.tail();
    assert_eq!(list.head(), None);
}

//对不可变引用迭代器的测试，和第二版一样
pub fn test2(){
    let list = List::new().append(1).append(2).append(3);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}