use std::cell::{RefCell, Ref};

#[derive(Debug)]
struct A{
    a:i32,
}

//测试Box所有权转移
pub fn box_test(){
    let q = 1;
    let w = Box::new(q);
    drop(q);//drop无效
    println!("{} {:p}",q,&q);//1 0xf7fb94
    println!("{} {:p}",w,&w);//1 0xf7fb98
    //看来Box并不会借用掉基本类型变量的所有权
    let x = A{a:2};
    let y = Box::new(x);
    println!("{} {:p}",y.a,&y); //2 0xf7fbf0
    // println!("{} {:p}",x.a,&x); //这里报错了，因为x已经被y借用了
}

//测试Vec所有权转移
pub fn vec_test(){
    let mut v:Vec<i32> = Vec::new();
    let a =1;
    let b =2;
    v.push(a);
    v.push(b);
    println!("{:?}",v);//[1, 2]
    println!("{}",a);//1
    let mut v1:Vec<A> = Vec::new();
    let a1 = A{a:3};
    let b1 = A{a:4};
    v1.push(a1);
    v1.push(b1);
    println!("{:?}",v1);//[A { a: 3 }, A { a: 4 }]
    // println!("{}",a1.a);//报错，a1的所有权被转移
}

//refcell测试
pub fn refcell_test(){
    let a =5;
    let b = RefCell::new(a);
    println!("{}",a);
    println!("{}",b.borrow());
    let a1 = A{a:6};
    let b1 = RefCell::new(a1);
    // println!("{:?}",a1);//报错，被借用了
    println!("{:?}",b1.borrow());
}