use std::rc::{Rc,Weak};
use std::cell::{RefCell,Ref,RefMut};
use std::sync::Arc;
use std::thread;

//Rc使用
pub fn rc_test1(){
    let a =Rc::new(List::Cons(5,Rc::new(List::Cons(10,Rc::new(List::Nil)))));
    println!("a={}",Rc::strong_count(&a)); //1
    let _b=List::Cons(3,Rc::clone(&a));
    println!("b={}",Rc::strong_count(&a)); //2
    {
        let _c=List::Cons(4,Rc::clone(&a));
        println!("c={}",Rc::strong_count(&a)); //3
    }
    println!("final {}",Rc::strong_count(&a)); //2
}

//Rc基本测试
pub fn rc_test1_1(){
    let a =Rc::new(5);
    println!("{} {} {:p}",a,*a,a);//5 5 0xee2830
}

enum List{
    Cons(i32,Rc<List>),
    Nil,
}

//RefCell使用例子
pub fn rc_test2(){
    let value =Rc::new(RefCell::new(5));
    let a=Rc::new(List1::Cons(Rc::clone(&value),Rc::new(List1::Nil)));
    let b=List1::Cons(Rc::new(RefCell::new(6)),Rc::clone(&a));
    let c=List1::Cons(Rc::new(RefCell::new(10)),Rc::clone(&a));
    *value.borrow_mut() +=10;
    println!("{:?}",a); //Cons(RefCell { value: 15 }, Nil)
    println!("{:?}",b); //Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
    println!("{:?}",c); //Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
}

//RefCell基本使用
pub fn rc_test2_1(){
    let c = RefCell::new(5);
    println!("{}",c.into_inner());//返回RefCell中包裹的值，5
    let c = RefCell::new(5);//上面的c已经move了，这里重新初始化
    let old_c = c.replace(6);
    println!("{} {}",c.into_inner(),old_c);//RefCell是不能直接打印的，6 5
    let p = RefCell::new(5);
    println!("{:p}",p.as_ptr());//0xf7fc20
}

#[derive(Debug)]
enum List1{
    Cons(Rc<RefCell<i32>>,Rc<List1>),
    Nil,
}

//Weak+Rc+RefCell
pub fn rc_test3() {
    let leaf=Rc::new(Node{
        value:3,
        parent:RefCell::new(Weak::new()),
        children:RefCell::new(vec![]),
    });
    println!("{:?}",leaf.parent.borrow().upgrade()); //None
    let branch =Rc::new(Node{
        value:5,
        parent:RefCell::new(Weak::new()),
        children:RefCell::new(vec![Rc::clone(&leaf)]),//branch连接leaf
    });
    *leaf.parent.borrow_mut()=Rc::downgrade(&branch);//这是Weak赋值的唯一写法！
    println!("{:?}",leaf.parent); //RefCell { value: (Weak) }
    println!("{:?}",leaf.parent.borrow().upgrade().unwrap()); //Node { value: 5, parent: RefCell { value: (Weak) },
    // children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } }
    let leaf1=Rc::new(Node{
        value:4,
        parent:RefCell::new(Weak::new()),
        children:RefCell::new(vec![]),
    });
    // *branch.children.borrow_mut()= vec![Rc::clone(&leaf1)]; //这里是更换leaf
    branch.children.borrow_mut().push(Rc::clone(&leaf1));
    println!("{:?}",branch);//成功添加了leaf，现在是3和4
}

#[derive(Debug)]
struct Node{
    value:i32,
    parent:RefCell<Weak<Node>>,
    children:RefCell<Vec<Rc<Node>>>,
}

//Ref和RefMut的使用，这两个是borrow和borrow_mut的返回值
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

//Arc使用
// 5
// 5
// 5
// 5
// 报错退出
pub fn rc_test5(){
    let five = Arc::new(5);
    println!("{}",five);
    for _ in 0..10 {
        let five = Arc::clone(&five);
        thread::spawn(move || {
            println!("{:?}", five);
        });
    }
}