use std::borrow::{Borrow, BorrowMut, Cow};

//Borrow和BorrowMut
#[allow(dead_code)]
pub fn b1() {
    //Borrow
    let s = "Hello".to_string();
    check(s); //Hello
    let s = "Hello";
    check(s); //Hello
    let b: Box<i32> = Box::new(5);
    let b1: Box<String> = Box::new("a".to_string());
    let c: &i32 = b.borrow(); //必须标注类型
    let c1: &String = b1.borrow();
    println!("{} {} {} {}", b, b1, c, c1); //5 a 5 a
    
    //BorrowMut
    let v = vec![1, 2, 3];
    check1(v);
    let mut b: Box<i32> = Box::new(6);
    let mut b1: Box<String> = Box::new("b".to_string());
    let c: &mut i32 = b.borrow_mut();
    let c1: &mut String = b1.borrow_mut();
    // println!("{} {}",b,b1);//这一行不能打印，因为已经有不可变引用了
    println!("{} {}", c, c1);
    let s: &str = "a";
    let ss: String = s.to_owned();
    let v: &[i32] = &[1, 2];
    let vv: Vec<i32> = v.to_owned();
    println!("{} {} {:?} {:?}", s, ss, v, vv);
}

fn check<T: Borrow<str>>(s: T) {
    assert_eq!("Hello", s.borrow());
}

fn check1<T: BorrowMut<[i32]>>(mut t: T) {
    assert_eq!(&mut [1, 2, 3], t.borrow_mut());
    t.borrow_mut().copy_within(1..2, 2);
    assert_eq!(&mut [1, 2, 2], t.borrow_mut());
}

//Cow
#[allow(dead_code)]
pub fn b2(){
    //写时clone
    fn abs_all(input:&mut Cow<[i32]>){
        for i in 0..input.len(){
            let v=input[i];
            if v<0{
                input.to_mut()[i]=-v;
            }
        }
    }
    // let slice = [-1,0,1];
    let slice =vec![-1,0,1];
    let mut input=Cow::from(&slice[..]);
    abs_all(&mut input);
    println!("{:?}",slice);//输出结果没变，写的时候clone了
    //结构体中使用Cow
    struct Items<'a,X:'a> where [X]:ToOwned<Owned = Vec<X>>{
        values:Cow<'a,[X]>
    }
    impl<'a,X:Clone+'a> Items<'a,X> where[X]:ToOwned<Owned=Vec<X>>{
        fn new(v:Cow<'a,[X]>)->Self{
            Items{values:v}
        }
    }
    let readonly=[1,2];
    let borrowed=Items::new((&readonly[..]).into());
    match borrowed{
        Items{values:Cow::Borrowed(b)}=>println!("borrowed {:?}", b),
         _ => panic!("expect borrowed value"),
    }
    let mut clone_on_write=borrowed;
    //to_mut获取可变引用
    clone_on_write.values.to_mut().push(3);
    match clone_on_write{
        Items{values:Cow::Owned(b)}=>println!("{:?}",b),//修改成功
        _=>panic!("expect owned data")
    }
}