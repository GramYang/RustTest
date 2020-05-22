use std::rc::{Rc,Weak};
use std::cell::{RefCell,Ref,RefMut};
use std::sync::Arc;
use std::thread;
use std::ops::Deref;

//Rc使用，clone和强引用数
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
    let b = Rc::new(RcBox{x:100});
    println!("{:?} {:?} {:p}",b,*b,b);//RcBox { x: 100 } RcBox { x: 100 } 0xf930f0
    //get_mut修改Rc包裹的值
    let mut x = Rc::new(3);
    *Rc::get_mut(&mut x).unwrap() = 4;
    assert_eq!(*x, 4);
    let _y = Rc::clone(&x);
    assert!(Rc::get_mut(&mut x).is_none());//x是一个shared值，修改他是不安全的，因此调用get_mut返回的是None
    //ptr_eq判断两个Rc是否指向同一个值
    let five = Rc::new(5);
    let same_five = Rc::clone(&five);
    let other_five = Rc::new(5);
    assert!(Rc::ptr_eq(&five, &same_five));
    assert!(!Rc::ptr_eq(&five, &other_five));
    //make_mut和get_mut类似，不同的是如果make_mut中的值变成了shared，make_mut会创建一个新值
    let mut data = Rc::new(5);
    *Rc::make_mut(&mut data) += 1;        // Won't clone anything
    let mut other_data = Rc::clone(&data);    // Won't clone inner data
    *Rc::make_mut(&mut data) += 1;        // Clones inner data
    *Rc::make_mut(&mut data) += 1;        // Won't clone anything
    *Rc::make_mut(&mut other_data) *= 2;  // Won't clone anything
// Now `data` and `other_data` point to different allocations.
    assert_eq!(*data, 8);
    assert_eq!(*other_data, 12);
    //make_mut配合Weak
    let mut data1 = Rc::new(75);
    let weak = Rc::downgrade(&data1);
    assert_eq!(75, *data1);
    assert_eq!(75, *weak.upgrade().unwrap());
    *Rc::make_mut(&mut data1) += 1;
    assert_eq!(76, *data1);
    assert!(weak.upgrade().is_none());//Weak被drop了
    //try_unwrap占用Rc后返回Rc中的值，如果其中的值是shared则返回Err
    let c = Rc::new(RcBox{x:1000});
    println!("{:?}",Rc::try_unwrap(c));//Ok(RcBox { x: 1000 })
    let c1 = Rc::new(RcBox{x:1001});
    let _c2 = Rc::clone(&c1);
    // drop(_c2);
    println!("{:?}",Rc::try_unwrap(c1));//Err(RcBox { x: 1001 })

}

#[derive(Debug)]
struct RcBox{
    x:i32
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
    //borrow, 可以有多个borrow
    let c = RefCell::new(5);
    let borrowed_five = c.borrow();
    let borrowed_five2 = c.borrow();
    //borrow_mut，只能有一个borrow_mut
    let c = RefCell::new(5);
    *c.borrow_mut() = 7;
    assert_eq!(*c.borrow(), 7);
    //borrow和borrow_mut不能共存
    // let result = thread::spawn(move || {
    //     let c = RefCell::new(5);
    //     let m = c.borrow_mut();
    //     let b = c.borrow(); // this causes a panic
    // }).join();
    // assert!(result.is_err());
    //get_mut和borrow_mut类似，区别是c必须是mut的，官方建议使用borrow_mut
    let mut c = RefCell::new(5);
    *c.get_mut() += 1;
    assert_eq!(c, RefCell::new(6));
    //into_inner消费RefCell，返回其中包裹的值
    let c = RefCell::new(5);
    let five = c.into_inner();
    //replace替换内部的值并返回旧值
    let cell = RefCell::new(5);
    let old_value = cell.replace(6);
    assert_eq!(old_value, 5);
    assert_eq!(cell, RefCell::new(6));
    //replace_with
    let cell = RefCell::new(5);
    let old_value = cell.replace_with(|&mut old| old + 1);
    assert_eq!(old_value, 5);
    assert_eq!(cell, RefCell::new(6));
    //swap替换两个RefCell的包裹值
    let c = RefCell::new(5);
    let d = RefCell::new(6);
    c.swap(&d);
    assert_eq!(c, RefCell::new(6));
    assert_eq!(d, RefCell::new(5));
    //try_borrow不可变借用内部值，如果事先borrow_mut了则返回Err
    let c = RefCell::new(5);
    {
        let m = c.borrow_mut();
        assert!(c.try_borrow().is_err());
    }
    {
        let m = c.borrow();
        assert!(c.try_borrow().is_ok());
    }
    //try_borrow_mut可变借用内部值，不能和borrow共存，只能borrow_mut一次
    let c = RefCell::new(5);
    {
        let m = c.borrow();
        assert!(c.try_borrow_mut().is_err());
    }
    assert!(c.try_borrow_mut().is_ok());
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
    *leaf.parent.borrow_mut()=Rc::downgrade(&branch);//Weak赋值写法
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


//weak基本使用
pub fn rc_test3_1(){
    let a:Weak<i32> = Weak::new();//这里必须要指定类型
    println!("{}", a.upgrade().is_none());//true
    let f = Rc::new(5);
    let weak_f = Rc::downgrade(&f);
    let strong_f = weak_f.upgrade();
    println!("{}",strong_f.is_some());//true
    let first = Weak::new();
    let second = Weak::new();
    println!("{}",first.ptr_eq(&second));//true
    let third_rc = Rc::new(());
    let third = Rc::downgrade(&third_rc);
    println!("{}",first.ptr_eq(&third));//false
}

//RefCell+Weak+Option
pub fn rc_test3_2(){
    let mut big = BigBox{b:Some(RefCell::new(Weak::new()))};
    let small = Rc::new(SmallBox{s:100});
    //因为unwrap会占用所有权，因此需要用.as_ref().unwrap()
    println!("{:?}",big.b.as_ref().unwrap().borrow().upgrade());//None
    *big.b.as_ref().unwrap().borrow_mut() = Rc::downgrade(&small);
    println!("{:?}",big.b.as_ref().unwrap().borrow().upgrade());//Some(SmallBox { s: 100 })
    println!("{:?}",big);//BigBox { b: Some(RefCell { value: (Weak) }) }
    //看来每次调用upgrade才会返回Weak的传入值，否则只会返回Weak
}

#[derive(Debug)]
struct BigBox{
    b:Option<RefCell<Weak<SmallBox>>>
}

#[derive(Debug)]
struct SmallBox{
    s:i32
}

//Ref和RefMut的粒子，这两个是borrow和borrow_mut的返回值
pub fn rc_test4(){
    let c=RefCell::new((5, 'b'));
    let b1:Ref<(i32,char)> = c.borrow();
    let b1_1 = *b1;
    let b2 = Ref::map(b1,|t|&t.0);
    // println!("{:?}",b1); //b1已经moved了
    println!("{:?}",b2); //5
    println!("{:?}",c); //RefCell { value: (5, 'b') }，Ref只是借用了RefCell
    println!("{:?}",c.borrow()); //(5, 'b')
    let x = RefCell::new((5,'b'));
    {
        let y1:RefMut<(u32,char)>=x.borrow_mut();
        let y1_1 = *y1;
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