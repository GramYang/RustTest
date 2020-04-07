
//函数泛型
pub fn g_test1() {
    let number_list=vec![34,50,25,100,65];
    let result=largest(&number_list);
    println!("{}",result); //100
    let char_list=vec!['y','m','a','q'];
    let result=largest(&char_list);
    println!("{}",result); //y
    let a = "abc";
    a.abc(); //基本类型也能实现trait，但是范围仅限于use了这个trait的作用域
}

fn largest<T:PartialOrd+Copy>(list:&[T])->T{
    let mut largest=list[0];
    for &item in list.iter(){
        if item>largest{
            largest=item;
        }
    }
    largest
}

trait ABC {
    fn abc(&self);
}

impl ABC for &str{
    fn abc(&self){
        println!("abc trait on &str");
    }
}

struct Point<T>{
    x:T,
    y:T,
}

impl<T> Point<T>{
    fn x(&self)->&T{
        &self.x
    }
}

//指定泛型方法
impl Point<f32>{
    fn distance(&self)->f32{
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }
}

struct Point1<T,U>{
    x:T,
    y:U,
}

impl<T,U> Point1<T,U>{
    fn mix<V,W>(self, other:Point1<V,W>) ->Point1<T,W>{
        Point1{
            x:self.x,
            y:other.y,
        }
    }
}

//结构体泛型
pub fn g_test2(){
    let a=Point{x:5,y:10};
    let _=Point{x:1.0,y:4.0};
    println!("{}",a.x());
    let p1=Point1{x:5,y:10.4};
    let p2=Point1{x:"Hello",y:'c'};
    let p3=p1.mix(p2); //p2的所有权转移进mix
    println!("{} {}",p3.x,p3.y);
}