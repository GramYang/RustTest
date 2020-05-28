use std::ops::Add;
use std::fmt;

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

