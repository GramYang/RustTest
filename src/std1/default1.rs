use std::cell::RefCell;
use std::rc::Rc;

//测试default使用
pub fn t1(){
    let options: SomeOptions = Default::default();
    println!("{:?}",options);//SomeOptions { foo: 0, bar: 0.0 }
    let options = SomeOptions { foo: 42, ..Default::default() };
    println!("{:?}",options);//SomeOptions { foo: 42, bar: 0.0 }
    let e:Entity = Default::default();
    println!("{:?}",e);//Entity { a: 0, b: "", c: None, d: [], e: RefCell { value: B }, f: []  }
}

#[derive(Default,Debug)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}

#[derive(Default,Debug)]
struct Entity{
    a:i32,
    b:String,
    c:Option<String>,
    d:Vec<i32>,
    e:Rc<RefCell<Kind>>,
    f:Funcs,
    // g:Func,//函数指针不能default，用Box包裹也不行
}

#[derive(Debug)]
enum Kind {
    A,
    B,
    C,
}

impl Default for Kind {
    fn default() -> Self { Kind::B }
}

type Funcs = Vec<Func>;
type Func = Box<fn(i32) ->i32>;