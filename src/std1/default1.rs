use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn d1() {
    let options: SomeOptions = Default::default();
    println!("{:?}", options);
    let options = SomeOptions {
        foo: 42,
        ..Default::default()
    };
    println!("{:?}", options);
    let e: Entity = Default::default();
    println!("{:?}", e);
}

#[allow(dead_code)]
#[derive(Default, Debug)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
struct Entity {
    a: i32,
    b: String,
    c: Option<String>,
    d: Vec<i32>,
    e: Rc<RefCell<Kind>>,
    f: Funcs,
    // g:Func,//函数指针不能default，用Box包裹也不行
}

#[allow(dead_code)]
#[derive(Debug)]
enum Kind {
    A,
    B,
    C,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::B
    }
}

type Funcs = Vec<Func>;
type Func = Box<fn(i32) -> i32>;
