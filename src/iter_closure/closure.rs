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

// type Bibao = dyn Fn(String)->String + 'static;
//
// //闭包函数指针
// pub fn close_test4(){
//     let a = "456";
//     let b1: Bibao = move |mut s:String|{
//         s.push_str(a);
//         s
//     };
//     let mut v = Vec::new();
//     op1(b1,&v);
//     v.get(0)(String::from("123"));
// }
//
// fn op1<T:Fn(String) ->String>(t:T,mut v:&Vec<T>){
//     v.push(t);
// }