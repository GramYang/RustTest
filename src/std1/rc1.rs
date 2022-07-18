use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

//Rc基本使用
#[allow(dead_code)]
pub fn r1() {
    #[derive(Debug)]
    struct RcBox {
        x: i32,
    }
    let a = Rc::new(5);
    println!("{} {} {:p}", a, *a, a); //5 5 0xee2830
    let b = Rc::new(RcBox { x: 100 });
    // let b1 = *b;//不能这么写，报错
    println!("{:?} {:?} {:p}", b, *b, b); //RcBox { x: 100 } RcBox { x: 100 } 0xf930f0
                                          //strong_count
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("a={}", Rc::strong_count(&a)); //1
    let _b = List::Cons(3, Rc::clone(&a));
    println!("b={}", Rc::strong_count(&a)); //2
    {
        let _c = List::Cons(4, Rc::clone(&a));
        println!("c={}", Rc::strong_count(&a)); //3
    }
    println!("final {}", Rc::strong_count(&a)); //2
                                                //get_mut修改Rc包裹的值
    let mut x = Rc::new(3);
    *Rc::get_mut(&mut x).unwrap() = 4;
    assert_eq!(*x, 4);
    let _y = Rc::clone(&x);
    assert!(Rc::get_mut(&mut x).is_none()); //x是一个shared值，修改他是不安全的，因此调用get_mut返回的是None
                                            //ptr_eq判断两个Rc是否指向同一个值
    let five = Rc::new(5);
    let same_five = Rc::clone(&five);
    let other_five = Rc::new(5);
    assert!(Rc::ptr_eq(&five, &same_five));
    assert!(!Rc::ptr_eq(&five, &other_five));
    //make_mut和get_mut类似，不同的是如果make_mut中的值变成了shared，make_mut会创建一个新值
    let mut data = Rc::new(5);
    *Rc::make_mut(&mut data) += 1; // Won't clone anything
    let mut other_data = Rc::clone(&data); // Won't clone inner data
    *Rc::make_mut(&mut data) += 1; // Clones inner data
    *Rc::make_mut(&mut data) += 1; // Won't clone anything
    *Rc::make_mut(&mut other_data) *= 2; // Won't clone anything
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
    assert!(weak.upgrade().is_none()); //Weak被drop了
                                       //try_unwrap占用Rc后返回Rc中的值，如果其中的值是shared则返回Err
    let c = Rc::new(RcBox { x: 1000 });
    println!("{:?}", Rc::try_unwrap(c)); //Ok(RcBox { x: 1000 })
    let c1 = Rc::new(RcBox { x: 1001 });
    let _c2 = Rc::clone(&c1);
    // drop(_c2);
    println!("{:?}", Rc::try_unwrap(c1)); //Err(RcBox { x: 1001 })
}

//Weak
#[allow(dead_code)]
pub fn r2() {
    let a: Weak<i32> = Weak::new(); //这里必须要指定类型
    println!("{}", a.upgrade().is_none()); //true
    let f = Rc::new(5);
    let weak_f = Rc::downgrade(&f);
    let strong_f = weak_f.upgrade();
    println!("{}", strong_f.is_some()); //true
    let first = Weak::new();
    let second = Weak::new();
    println!("{}", first.ptr_eq(&second)); //true
    let third_rc = Rc::new(());
    let third = Rc::downgrade(&third_rc);
    println!("{}", first.ptr_eq(&third)); //false
}

//Weak+Rc+RefCell的树结构
#[allow(dead_code)]
pub fn r3() {
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("{:?}", leaf.parent.borrow().upgrade()); //None
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]), //branch连接leaf
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); //Weak赋值写法
    println!("{:?}", leaf.parent); //RefCell { value: (Weak) }
    println!("{:?}", leaf.parent.borrow().upgrade().unwrap()); //Node { value: 5, parent: RefCell { value: (Weak) },
                                                               // children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } }
    let leaf1 = Rc::new(Node {
        value: 4,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // *branch.children.borrow_mut()= vec![Rc::clone(&leaf1)]; //这里是更换leaf
    branch.children.borrow_mut().push(Rc::clone(&leaf1));
    println!("{:?}", branch); //成功添加了leaf，现在是3和4
}

//RefCell+Weak+Option
#[allow(dead_code)]
pub fn r4() {
    #[derive(Debug)]
    struct BigBox {
        b: Option<RefCell<Weak<SmallBox>>>,
    }

    #[derive(Debug)]
    struct SmallBox {
        s: i32,
    }
    let big = BigBox {
        b: Some(RefCell::new(Weak::new())),
    };
    let small = Rc::new(SmallBox { s: 100 });
    //因为unwrap会占用所有权，因此需要用.as_ref().unwrap()
    println!("{:?}", big.b.as_ref().unwrap().borrow().upgrade()); //None
    *big.b.as_ref().unwrap().borrow_mut() = Rc::downgrade(&small);
    println!("{:?}", big.b.as_ref().unwrap().borrow().upgrade()); //Some(SmallBox { s: 100 })
    println!("{:?}", big); //BigBox { b: Some(RefCell { value: (Weak) }) }
                           //看来每次调用upgrade才会返回Weak的传入值，否则只会返回Weak
}
