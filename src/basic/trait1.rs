use std::{fmt, mem};


//trait
#[allow(dead_code)]
pub fn t1(){
    //多态
    let p=Human;
    p.fly(); //human fly
    Pilot::fly(&p); //pilot fly
    Wizard::fly(&p); //wizard fly
    p.foo(); //Pilot接口默认实现：foo。有默认实现的trait相当于父类！
    println!("{}",Dog::baby_name()); //Spot
    println!("{}",<Dog as Animal>::baby_name()); //puppy，这是完全限定语法
    let p=Point1{x:1,y:2};
    p.outline_print();
}

trait Pilot{
    fn fly(&self);
    fn foo(&self){
        println!("Pilot接口默认实现：foo")
    }
}

trait Wizard{
    fn fly(&self);
}

struct Human;

impl Pilot for Human{
    fn fly(&self){
        println!("pilot fly");
    }
}

impl Human{
    fn fly(&self){
        println!("human fly")
    }
}

impl Wizard for Human{
    fn fly(&self){
        println!("wizard fly");
    }
}

trait Animal{
    fn baby_name()->String;
}

struct Dog;

impl Dog{
    fn baby_name()->String{
        String::from("Spot")
    }
}

impl Animal for Dog{
    fn baby_name()->String{
        String::from("puppy")
    }
}

struct Point1{
    x:i32,y:i32
}

trait OutlinePrint: fmt::Display{
    fn outline_print(&self){
        //to_string是Display的方法，其中调用format_args!("{}", self)会用到fmt::Display的实现
        let output:String=self.to_string();
        let len = output.len();
        println!("{}","*".repeat(len+4));
        println!("*{}*"," ".repeat(len+2));
        println!("* {} *",output);
        println!("*{}*"," ".repeat(len+2));
        println!("{}","*".repeat(len+4));
    }
}

impl OutlinePrint for Point1{}

impl fmt::Display for Point1{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        write!(f,"({}, {})",self.x,self.y)
    }
}

//trait实现
#[allow(dead_code)]
pub fn t2(){
    let duck = Duck;
    let p_duck = &duck;
    let p_bird = p_duck as &dyn Bird;
    println!("Size of p_duck {}, Size of p_bird {}", mem::size_of_val(&p_duck), mem::size_of_val(&p_bird));
    let duck_fly= Duck::fly as usize;
    let swan_fly  = Swan::fly as usize;
    println!("Duck::fly {}", duck_fly);
    println!("Swan::fly {}", swan_fly);
    print_traitobject(p_bird);
    let swan = Swan;
    print_traitobject(&swan as &dyn Bird);
}

trait Bird {
    fn fly(&self);
}
struct Duck;
struct Swan;

impl Bird for Duck {
    fn fly(&self) { println!("duck duck"); }
}

impl Bird for Swan {
    fn fly(&self) { println!("swan swan");}
}

// 参数是 trait object 类型，p 是一个胖指针
fn print_traitobject(p: &dyn Bird) {
    // 使用transmute执行强制类型转换，把变量p的内部数据取出来
    let (data, vtable) : (usize, usize) = unsafe {mem::transmute(p)};
    println!("TraitObject    [data:{}, vtable:{}]", data, vtable);
    unsafe {
        // 使用as执行强制类型转换，将vtable从 `usize` 类型转为 `*const usize` 类型
        let v: * const usize = vtable as * const () as * const usize;
        // 打印出指针 v 指向的内存区间的值
        println!("data in vtable [{}, {}, {}, {}]",
                 *v, *v.offset(1), *v.offset(2), *v.offset(3));
    }
}
