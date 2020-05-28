use std::rc::Rc;
use std::cell::RefCell;

pub fn fn_test1() {
    let a = [1, 2, 3, 4, 8, 9];
    println!("There is 7 in the array: {}", find(7, &a)); //false
    println!("There is 8 in the array: {}", find(8, &a)); //true
    let (x,y) = two(2);
    println!("返回元组: {} {}",x,y);
}

fn find(n: i32, a: &[i32]) -> bool {
    for i in a {
        if *i == n {
            return true;
        }
    }
    false
}

fn two(n: i32) -> (i32, i32) {
    (n*n, n*n*n)
}

//空返回类型测试。函数无返回类型也是可以return的，返回的是()，return时会执行附带的函数或者宏。
pub fn fn_test2(){
    t1();
}

fn t1(){
    let x = 5;
    match x{
        0..=5 => return println!("bingo"), //bingo
        _ => println!("miss"),
    }
    println!("没有返回！");
    return panic!("用来返回!");
}

//dyn和impl的使用
pub fn fn_test3(){
    let b = Boob{a:100};
    let b1 = impl_test(b);
    // println!("{:?}",b1);//报错，impl ImplTest不能打印
    trait Trait {}
    impl Trait for i32 {}
    fn function2() -> Box<dyn Trait> {
        Box::new(1)
    }//Box内使用trait必须要加dyn来表示Trait是一个trait
}

struct Boob{
    a:i32,
}

trait ImplTest{
    fn it(&self);
}

impl ImplTest for Boob{
    fn it(&self){
        println!("{}", self.a);
    }
}

fn impl_test(x:impl ImplTest) -> impl ImplTest{
    x.it();
    return x;
}

//函数指针能否引用外部变量？不能！
pub fn f_t4(){
    // println!("{}",big(1,String::from("a"))(2,String::from("b")));
}

// fn big(a:i32,b:String) -> fn(i32,String)->String{
//     fn small(c:i32,d:String) ->String{
//         let mut s=String::new();
//         s.push((a+c).to_string());//报错，不能捕获a
//         s.push_str(&b);
//         s.push_str(&d);
//         s
//     }
// }

type fp = fn(a:String)->String;

fn fp1(a:String)->String{
    let mut s = String::new();
    s = a.clone();
    s
}

fn fp2(a:String)->String{
    let mut s = String::new();
    s = a.clone();
    s.push_str(&a);
    s
}

//函数指针及其实现的赋值和封装，可以不用Box封装就存入容器
pub fn ft5(){
    let mut arr:Vec<fp> = vec![];
    arr.push(fp1);
    arr.push(fp2);
    let a1 = arr.get(0).unwrap();
    let a2 = arr.get(1).unwrap();
    println!("{} {}", a1("123".to_string()),a2("456".to_string()));
}