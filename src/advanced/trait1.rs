use std::ops::Add;
use std::{fmt,mem};
use futures::core_reexport::fmt::Display;

#[derive(Debug,PartialEq)]
struct Point{
    x:i32,y:i32,
}

impl Add for Point{
    type Output = Point;
    fn add(self,other:Point)->Point{
        Point{
            x:self.x+other.x,y:self.y+other.y,
        }
    }
}

#[derive(PartialEq,Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters{
    type Output=Millimeters;
    fn add(self,other:Meters)->Millimeters{
        Millimeters(self.0+(other.0*1000))
    }
}

//运算符重载
pub fn t_test1(){
    assert_eq!(Point{x:1,y:0}+Point{x:2,y:3},Point{x:3,y:3});
    assert_eq!(Millimeters(1)+Meters(1),Millimeters(1001));
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

//trait多态
pub fn t_test2(){
    //情况1
    let p=Human;
    p.fly(); //human fly
    Pilot::fly(&p); //pilot fly
    Wizard::fly(&p); //wizard fly
    p.foo(); //Pilot接口默认实现：foo。有默认实现的trait相当于父类！
    //情况2
    println!("{}",Dog::baby_name()); //Spot
    println!("{}",<Dog as Animal>::baby_name()); //puppy，这是完全限定语法
    //父trait
    let p=Point1{x:1,y:2};
    p.outline_print();
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

//newtype模式
pub fn t_test3(){
    let w=Wrapper(vec![String::from("hello"),String::from("world")]);
    println!("{}",w); //[hello, world]
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper{
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        write!(f,"[{}]",self.0.join(", "))
    }
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

//测试trait的实现
// 从这里的分析结果可以看到，Rust的动态分派，和C++的动态分派，内存布局有所不同。在C++里，如果一个类型里面有虚函数，
// 那么每一个这种类型的变量，内部都包含一个指向虚函数表的地址。而在 Rust 里面，对象本身不包含指向虚函数表的指针，
// 这个指针是存在于 trait object 指针里面。如果一个类型实现了多个 trait，
// 那么不同的 trait object 指向的虚函数表也不一样。
pub fn tt4(){
    let duck = Duck;
    let p_duck = &duck;
    let p_bird = p_duck as &dyn Bird;
    println!("Size of p_duck {}, Size of p_bird {}", mem::size_of_val(&p_duck), mem::size_of_val(&p_bird));
    //Size of p_duck 8, Size of p_bird 16
    let duck_fly= Duck::fly as usize;
    let swan_fly  = Swan::fly as usize;
    println!("Duck::fly {}", duck_fly);//Duck::fly 4200512
    println!("Swan::fly {}", swan_fly);//Swan::fly 4200608
    print_traitobject(p_bird);
    let swan = Swan;
    print_traitobject(&swan as &dyn Bird);
    // TraitObject    [data:17431216, vtable:6332664]
    // data in vtable [4203872, 0, 1, 4200512]
    // TraitObject    [data:17431520, vtable:6332880]
    // data in vtable [4203856, 0, 1, 4200608]
}

trait Foo{
    fn foo(&self);
}

impl Foo for i32 {
    fn foo(&self) { println!("{}", self); }
}

trait Double:Display {
    fn new() -> Self where Self: Sized;
    fn double(&mut self);
}

impl Double for i32 {
    fn new() -> Self{ 0 }
    fn double(&mut self) { *self *= 2; }
}

// Trait Object测试
//Rust禁止使用trait object来调用泛型函数，泛型函数是从虚函数表中剔除掉了的。
pub fn tt5(){
    //如果给Foo加上where Self: Sized，那么p.foo();就会报cannot be made into an object
    let x = 1_i32;
    x.foo();//1
    let p = &x as &dyn Foo;
    p.foo();//1
    //这里如果去掉Double::new()的where Self: Sized约束就会报错
    let mut i = 1;
    let p : &mut dyn Double = &mut i as &mut dyn Double;
    p.double();
    println!("{}",p);//2
}