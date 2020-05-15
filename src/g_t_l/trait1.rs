use std::fmt::Display;
use std::net::TcpListener;

//trait使用
pub fn t_test1(){
    let na = NewsArticle{
        headline:String::from("1"),
        location:String::from("2"),
        author:String::from("3"),
        content:String::from("4"),
    };
    println!("{}",na.summarize()); //1, by 3 (2)
    println!("{}",na.summarize1()); //(Read more...)
    notify1(&na); //trait参数
    notify2(&na); //trait bound
    let _=notify3(); //trait返回值
}

trait Summary{
    fn summarize(&self)->String;
    fn summarize1(&self)->String{ //trait默认实现
        String::from("(Read more...)")
    }
}

struct NewsArticle{
    headline:String,
    location:String,
    author:String,
    content:String,
}

impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("{}, by {} ({})",self.headline,self.author,self.location)
    }
}

//item可以调用Summary的默认方法，也可以调用Summary实现的实例
fn notify1(item: &impl Summary) {
    println!("{}",item.summarize());
}

//trait bound
fn notify2<T:Summary>(item:&T){
    println!("{}",item.summarize());
}

//trait返回值
fn notify3()->impl Summary{
    NewsArticle{
        headline:String::from("5"),
        location:String::from("6"),
        author:String::from("7"),
        content:String::from("8"),
    }
}

struct Pair<T> {
    x:T, y:T,
}

//泛型impl块
impl<T> Pair<T> {
    fn new(x:T,y:T)->Self{
        Self{
            x, y,
        }
    }
}

//泛型impl块指定trait
impl<T:Display+PartialOrd> Pair<T> {
    fn cmd(&self){
        if self.x>=self.y{
            println!("{}",self.x)
        }else{
            println!("{}",self.y)
        }
    }
}

//给内置类型实现trait
pub fn t_test2(){
    let a1 = "1234";
    a1.bt(); //内置基本类型也可以实现trait!
}

trait BasicTrait {
    fn bt(&self);
}

impl BasicTrait for &str{
    fn bt(&self){
        println!("内置基本类型也可以实现trait!")
    }
}

//结构体内嵌trait，只能用泛型约束
pub fn t_test3(){
    let b = Big{a:Some(1),b:Some(String::from("123")),c:Small{d:Some(2),e:Some(true)}};
    b.c.n();
}

trait Nested{
    fn n(&self);
}

struct Big<T:Nested> {
    a:Option<i32>,
    b:Option<String>,
    c:T,
}

struct Small{
    d:Option<i32>,
    e:Option<bool>,
}

impl Nested for Small{
    fn n(&self) {
        println!("this is Small, {:?} {:?}",self.d,self.e);//this is Small, Some(2) Some(true)
    }
}