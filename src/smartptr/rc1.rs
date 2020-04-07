use std::rc::{Rc,Weak};
use std::cell::{RefCell,Ref,RefMut};

//Rc使用
pub fn rc_test1(){
    let a =Rc::new(List::Cons(5,Rc::new(List::Cons(10,Rc::new(List::Nil)))));
    println!("a={}",Rc::strong_count(&a)); //1
    let b=List::Cons(3,Rc::clone(&a));
    println!("b={}",Rc::strong_count(&a)); //2
    {
        let c=List::Cons(4,Rc::clone(&a));
        println!("c={}",Rc::strong_count(&a)); //3
    }
    println!("final {}",Rc::strong_count(&a)); //2
}

enum List{
    Cons(i32,Rc<List>),
    Nil,
}

//RefCell使用
pub fn rc_test2(){
    let value=Rc::new(RefCell::new(5));
    let a=Rc::new(List1::Cons(Rc::clone(&value),Rc::new(List1::Nil)));
    let b=List1::Cons(Rc::new(RefCell::new(6)),Rc::clone(&a));
    let c=List1::Cons(Rc::new(RefCell::new(10)),Rc::clone(&a));
    *value.borrow_mut()+=10;
    println!("{:?}",a); //Cons(RefCell { value: 15 }, Nil)
    println!("{:?}",b); //Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
    println!("{:?}",c); //Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
}

#[derive(Debug)]
enum List1{
    Cons(Rc<RefCell<i32>>,Rc<List1>),
    Nil,
}

//Weak使用
pub fn rc_test3() {
    let leaf=Rc::new(Node{
        value:3,
        parent:RefCell::new(Weak::new()),
        children:RefCell::new(vec![]),
    });
    println!("{:?}",leaf.parent.borrow().upgrade()); //leaf parent = None
    let branch =Rc::new(Node{
        value:5,
        parent:RefCell::new(Weak::new()),
        children:RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut()=Rc::downgrade(&branch);
    println!("{:?}",leaf.parent); //RefCell { value: (Weak) }
    println!("{:?}",leaf.parent.borrow().upgrade()); //Some(Node { value: 5, parent: RefCell { value: (Weak) },
    // children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
}

#[derive(Debug)]
struct Node{
    value:i32,
    parent:RefCell<Weak<Node>>,
    children:RefCell<Vec<Rc<Node>>>,
}

//Ref和RefMut的使用
pub fn rc_test4(){
    let c=RefCell::new((5, 'b'));
    let b1:Ref<(i32,char)> = c.borrow();
    let b2 = Ref::map(b1,|t|&t.0);
    // println!("{:?}",b1); //b1已经moved了
    println!("{:?}",b2); //5
    println!("{:?}",c); //RefCell { value: (5, 'b') }，Ref只是借用了RefCell
    println!("{:?}",c.borrow()); //(5, 'b')
    let x = RefCell::new((5,'b'));
    {
        let y1:RefMut<(u32,char)>=x.borrow_mut();
        let mut y2:RefMut<u32>=RefMut::map(y1,|t|&mut t.0);
        println!("{}",y2); //5
        *y2 = 42;
    }
    println!("{:?}",*x.borrow()); //(42, 'b')
}