use std::borrow::{Borrow, BorrowMut};

//测试borrow的三个trait
//borrow和RefCell的区别：BorrowMut必须要给目标变量加上mut
pub fn bt1(){
    //Borrow
    //String有Borrow实现
    fn check<T: Borrow<str>>(s: T) {
        assert_eq!("Hello", s.borrow());
        println!("{}",s.borrow());//String可以直接borrow()，不需要标注类型
    }
    let s = "Hello".to_string();
    check(s);
    let s = "Hello";
    check(s);
    //Box和String不同
    let b:Box<i32> = Box::new(5);
    let b1:Box<String> = Box::new("a".to_string());
    let c:&i32 = b.borrow();//必须标注类型
    let c1:&String = b1.borrow();
    println!("{} {} {} {}",b,b1,c,c1);
    //BorrowMut
    //Vec
    fn check1<T: BorrowMut<[i32]>>(mut v: T) {
        assert_eq!(&mut [1, 2, 3], v.borrow_mut());
        v.borrow_mut().copy_within(1..2,2);
        assert_eq!(&mut [1, 2, 2], v.borrow_mut());
    }
    let v = vec![1, 2, 3];
    check1(v);
    //Box
    let mut b:Box<i32> = Box::new(6);//和RefCell的区别在此
    let mut b1:Box<String> = Box::new("b".to_string());
    let c:&mut i32 = b.borrow_mut();
    let c1:&mut String = b1.borrow_mut();
    // println!("{} {}",b,b1);//这一行不能打印，因为已经有不可变引用了
    println!("{} {}",c,c1);
    //ToOwned，每一种类型的to_owned()都只返回特定的类型
    let s: &str = "a";
    let ss: String = s.to_owned();
    let v: &[i32] = &[1, 2];
    let vv: Vec<i32> = v.to_owned();
}

