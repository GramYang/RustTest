use std::thread;
use std::time::Duration;

//闭包使用
pub fn close_test1() {
    let a=10;
    let b =7;
    generate_workout(a,b);
    let c=|x|{x+1};
    //闭包类型推断
    println!("{}",c(1)); //2，这个时候闭包c中参数x的类型固定为i32，用其他的类型将会报错
}

fn generate_workout(a:u32,b:u32){
    let expensive_closure=|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if a<25{
        println!("Today, do {} pushups!", expensive_closure(a));
        println!("Next, do {} situps!",expensive_closure(a))
    } else {
        println!("Today, run for {} minutes!",expensive_closure(a));
    }
}

//泛型函数闭包
pub fn close_test2(){
    let mut c=Cacher::new(|a|a);
    println!("{}",c.value(1)); //1
    println!("{}",c.value(2)); //1，为什么还会输出1？代码逻辑问题。这个时候需要用take()。
    //这里必须重新新建一个Cacher的实例来存放新的闭包，直接修改c.calculation报错：显示有两个不同的闭包
    let x:u32 = 100;
    let mut c = Cacher::new(move |a| a+x);
    println!("{}",(c.calculation)(100));
}

struct Cacher<T> where T:Fn(u32)->u32{ //Fn是闭包函数类型
    calculation:T,
    value:Option<u32>,
}

impl<T> Cacher<T> where T:Fn(u32)->u32{
    fn new(cal:T)->Cacher<T>{
        Cacher{
            calculation:cal,
            value:None,
        }
    }
    fn value(&mut self, arg:u32)->u32{
        match self.value{
            Some(v)=>v,
            None=>{
                let v=(self.calculation)(arg);
                self.value=Some(v);
                v
            },
        }
    }
}

//闭包捕获环境
pub fn close_test3() {
    let x=4;
    let equal_to_x=|z|z==x;
    println!("{}",equal_to_x(x)); //该闭包不可变的借用了x
    println!("{}",x); //4，还有效
    let x1=vec![1,2,3];
    let equal_to_x1=move |z|z==x1;
    println!("{}",equal_to_x1(vec![1,2,3])); //true
    // println!("{:?}",x1); //报错，x1被借用
}

//闭包三个trait测试
pub fn c_t4(){
    //Fn
    fn call_with_one<F>(func: F) -> usize
        where F: Fn(usize) -> usize {
        func(1)
    }
    let double = |x| x * 2;
    assert_eq!(call_with_one(double), 2);
    //FnMut
    let mut x = 5;
    {
        let mut square_x = || x *= x;
        square_x();
    }
    assert_eq!(x, 25);
    fn do_twice<F>(mut func: F)
        where F: FnMut()
    {
        func();
        func();
    }
    let mut x: usize = 1;
    {
        let add_two_to_x = || x += 2;
        do_twice(add_two_to_x);
    }
    assert_eq!(x, 5);
    //FnOnce
    fn consume_with_relish<F>(func: F)
        where F: FnOnce() -> String
    {
        // `func` consumes its captured variables, so it cannot be run more
        // than once.
        println!("Consumed: {}", func());
        println!("Delicious!");
        // Attempting to invoke `func()` again will throw a `use of moved
        // value` error for `func`.
    }
    let x = String::from("x");
    let consume_and_return_x = move || x;
    consume_with_relish(consume_and_return_x);
// `consume_and_return_x` can no longer be invoked at this point
}

//这里是FnOnce，但是Fn也行，因为FnOnce是Fn的父trait
fn big(a:i32,b:String) -> impl Fn(i32, String) -> String {
    move |c,d| {
        let mut s = String::new();
        s.push_str((a+c).to_string().as_str());
        s.push_str(&b.as_str());
        s.push_str(&d.as_str());
        s
    }
}

type Boo = Box<dyn Fn(i32) -> String>;

fn op1(x:i32, y:String) -> impl Fn(i32) -> String {
    move |s:i32| {
        let mut a = String::new();
        a.push_str((x+s).to_string().as_str());
        a.push_str(y.as_str());
        a
    }
}

fn op2(x:i32) -> impl Fn(i32) -> String{
    move |s:i32| {
        let mut a = String::new();
        a.push_str((x+s).to_string().as_str());
        a
    }
}

fn op3(y:String) ->impl Fn(i32) -> String{
    move |s:i32| {
        let mut a = String::new();
        a.push_str((s).to_string().as_str());
        a.push_str(y.as_str());
        a
    }
}

//Fn可以在Fn,FnMut,FnOnce之间切换，美汁儿汁儿
fn op4() ->impl Fn(i32) ->String{
    |s| {
        let mut a = String::new();
        a.push_str((s).to_string().as_str());
        a
    }
}

//用Fn实现golang的函数指针
pub fn c_t5(){
    //实现了golang的函数返回函数指针
    println!("{}",big(1,String::from("a"))(2,String::from("b")));//3ab
    println!("{}",big(3,String::from("c"))(4,String::from("d")));//7cd
    //用容器存储Fn
    let c1 = Box::new(op1(10,String::from("ab")));
    let c2 = Box::new(op2(20));
    let c3 = Box::new(op3(String::from("cd")));
    let c4 = Box::new(op4());
    let mut vec:Vec<Boo> = Vec::new();
    vec.push(c1);
    vec.push(c2);
    vec.push(c3);
    vec.push(c4);
    println!("{} {} {} {}",vec.get(0).unwrap()(30),vec.get(1).unwrap()(30),
             vec.get(2).unwrap()(30),vec.get(3).unwrap()(30));//40ab 50 30cd 30
}