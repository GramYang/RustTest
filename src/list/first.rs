use std::mem;

struct List{
    head:Link,
}

enum Link{
    Empty,
    More(Box<Node>),
}

struct Node{
    elem:i32,
    next:Link,
}

impl List{
    fn new() -> Self{
        List{head:Link::Empty}
    }

    //元素推入头部
    fn push(&mut self, elem:i32){
        let new_node=Box::new(Node{
            elem,
            next:mem::replace(&mut self.head,Link::Empty),
        });
        self.head=Link::More(new_node);
    }

    //取头部元素
    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next; //head后移
                Some(node.elem)
            }
        }
    }
}

//这里必须要实现Drop，因为在删除Box<Node>时不会递归的让Node先删除自己。
//因此我们必须手动的写出List的迭代drop来讲Node从Box中吊出来。其实就是删除Box<Node>。
impl Drop for List {
    //将head和全部的next全部设置为Link::Empty
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

//第一版的链表
pub fn test(){
    let mut list = List::new();

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}