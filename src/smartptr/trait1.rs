use std::ops::Deref;
use std::mem::drop;

//解引用、Deref trait使用
pub fn t_test1(){
    let x=5;
    let y=Box::new(x);
    let z=&x;
    assert_eq!(5,x);
    assert_eq!(5,*y);
    assert_eq!(5,*z);
    println!("{:p} {:p}",y,z) //0x21ea3d439d0 0x246c6ff29c
}

//自定义智能指针
pub fn t_test2(){
    let x=5;
    let y=MyBox::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y);
    // println!("{:p}",y); //不能打印地址
    //解引用强制多态
    let m=MyBox::new(String::from("Rust"));
    hello(&m); //可以看见类型默认转换了！
    // hello(&(*m)[..]); //和上面写法等效

}

struct MyBox<T>(T); //包含一个元素的元组结构体

impl<T> MyBox<T>{
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}

//实现解引用
impl<T> Deref for MyBox<T>{
    type Target=T;
    fn deref(&self)->&T{
        &self.0
    }
}

fn hello(name:&str){
    println!("{}",name);
}

//Drop trait
pub fn t_test3(){
    let a=CustomSmartPointer{data:String::from("1")}; //注意：这里的a不能_，因为一旦_就删掉了
    let b=CustomSmartPointer{data:String::from("2")};
    println!("created.");
}

struct CustomSmartPointer{
    data:String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("{}",self.data);
    }
}

//std::mem::drop函数手动drop
pub fn t_test4(){
    let c=CustomSmartPointer{data:String::from("some data")};
    println!("created");
    drop(c);
    println!("end");
}