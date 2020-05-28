use std::any::{Any, TypeId};
use std::fmt::Debug;

//这是rustprimer上的反射例子
pub fn r_t1(){
    let cfp = "/etc/wayslog.conf".to_string();
    assert_eq!(load_config(&cfp), vec!["/etc/wayslog.conf".to_string()]);
    let cfps = vec!["/etc/wayslog.conf".to_string(),
                    "/etc/wayslog_sec.conf".to_string()];
    assert_eq!(load_config(&cfps),
               vec!["/etc/wayslog.conf".to_string(),
                    "/etc/wayslog_sec.conf".to_string()]);
}

fn load_config<T:Any+Debug>(value:&T)->Vec<String>{
    let mut cfgs:Vec<String>=vec![];
    let value = value as &dyn Any;
    match value.downcast_ref::<String>(){//downcast_ref，如果value是String类型就返回&String，否则返回None
        Some(cfp) => cfgs.push(cfp.clone()),
        None => (),
    };
    match value.downcast_ref::<Vec<String>>() {
        Some(v) => cfgs.extend_from_slice(&v),
        None =>(),
    }

    if cfgs.len() == 0 {
        panic!("No Config File");
    }
    cfgs
}

//是否能判断一个实例实现了某个trait？只能用match
pub fn r_t2(){
    let a = Ba{a:100};
    println!("{:?} {:?}",TypeId::of::<dyn Fa>(), a.type_id());
    //TypeId { t: 4983854625466413765 } TypeId { t: 10339151103279636307 }
    println!("{:?} {:?}",TypeId::of::<Box<dyn Fa>>(), TypeId::of::<Box<Ba>>());
    //TypeId { t: 18112260363541837233 } TypeId { t: 7166434506009053646 }
    match a {
        Fa=>println!("Fa"),//Fa
    }
}

trait Fa{
    fn f(&self);
}

struct Ba{
    a:i32,
}

impl Fa for Ba{
    fn f(&self){
        println!("{}",self.a);
    }
}

//trait与实现转换测试
pub fn rt3(){
    //将实现了同一个trait的结构体装入同一个容器？可行
    let mut vec:Vec<Box<dyn Father>> = vec![];
    vec.push(Box::new(Son1{age:10}));
    vec.push(Box::new(Son2{age:20}));
    println!("{} {}",vec.get(0).unwrap().op1(), vec.get(1).unwrap().op1());
    //取出来之后，判断dyn trait是哪一个实现结构体类型？不行，用match也不行。
    println!("{:?} {:?}",vec.get(0).unwrap().type_id(), TypeId::of::<Son1>());
    //只能用枚举来表示trait，枚举值表示trait实现，但是这种方法不现实，因为不能动态添加枚举成员，除非你用宏来动态生成枚举
    let mut vec:Vec<Father1> = vec![];
    vec.push(Father1::Son1(10));
    vec.push(Father1::Son2(20));
    vec.push(Father1::Son3(30));
    println!("{:?} {:?} {:?}",vec.get(0).unwrap(), vec.get(1).unwrap(),vec.get(2).unwrap());
    match vec.get(0).unwrap(){
        &Father1::Son1(x) =>println!("this is Son1, age {}",x),
        &Father1::Son2(x) =>println!("this is Son1, age {}",x),
        &Father1::Son3(x) =>println!("this is Son1, age {}",x),
    };
    //用Any试一下，成功了！！
    let mut vec:Vec<Box<dyn Any>> = vec![];
    vec.push(Box::new(Son3{age:40}));
    vec.push(Box::new(Son4{age:50}));
    println!("{} {}",vec.get(0).unwrap().downcast_ref::<Son3>().unwrap().age,
             vec.get(1).unwrap().downcast_ref::<Son4>().unwrap().age);
    //再来一个泛型版的，因为Vec和Any都是长度不定的，因此都需要用Box套娃
    let mut vec:Vec<Box<dyn Any>> = vec![];
    vec.push(Box::new(Son3{age:60}));
    vec.push(Box::new(Son4{age:70}));
    let bb = BigBag{a:vec};
    println!("{} {}",bb.a.get(0).unwrap().downcast_ref::<Son3>().unwrap().age,
             bb.a.get(1).unwrap().downcast_ref::<Son4>().unwrap().age);
}

trait Father{
    fn op1(&self) ->i32;
}

struct Son1{
    age:i32,
}

impl Father for Son1{
    fn op1(&self) ->i32{
        self.age
    }
}

struct Son2{
    age:i32,
}

impl Father for Son2{
    fn op1(&self) ->i32{
        self.age
    }
}

#[derive(Debug)]
enum Father1{
    Son1(i32),Son2(i32),Son3(i32)
}

struct Son3{
    age:i32,
}

struct Son4{
    age:i32,
}

struct BigBag<T>{
    a:Vec<T>,
}
