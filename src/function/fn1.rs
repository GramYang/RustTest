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

//函数返回Rc+RefCell，Rc+RefCell作为函数参数
pub fn fn_test3() {
    let x = ret_rc();
    println!("{} {}",x.borrow().a,Rc::strong_count(&x));//100 1
    x.borrow_mut().a +=10;
    println!("{} {}",x.borrow().a,Rc::strong_count(&x));//110 1
    para_rc(Rc::clone(&x));//120 2 from para_rc
    println!("{} {}",x.borrow().a,Rc::strong_count(&x));//120 1
    let y = pr_rc(Rc::clone(&x));
    println!("{} {}",x.borrow().a,Rc::strong_count(&x));//220 2
    println!("{} {}",y.borrow().a,Rc::strong_count(&y));//220 2
    println!("{}",Rc::ptr_eq(&x,&y));//true
}

#[derive(Debug)]
struct Boob{
    a:i32,
}

//作为Rc返回的实例是不能修改的，需要套一层RefCell
fn ret_rc() -> Rc<RefCell<Boob>>{
    Rc::new(RefCell::new(Boob{a:100}))
}

//Rc作为参数
fn para_rc(x:Rc<RefCell<Boob>>){
    x.borrow_mut().a += 10;
    println!("{} {} from para_rc",x.borrow().a,Rc::strong_count(&x));
}

//Rc既作为参数又作为返回值
fn pr_rc(x:Rc<RefCell<Boob>>) -> Rc<RefCell<Boob>>{
    x.borrow_mut().a +=100;
    return x;
}

//dyn和impl的使用
pub fn fn_test4(){
    let b = Boob{a:100};
    let b1 = impl_test(b);
    // println!("{:?}",b1);//报错，impl ImplTest不能打印
    trait Trait {}
    impl Trait for i32 {}
    fn function2() -> Box<dyn Trait> {
        Box::new(1)
    }//Box内使用trait必须要加dyn来表示Trait是一个trait
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
pub fn f_t5(){
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