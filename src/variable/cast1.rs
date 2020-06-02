use std::any::{Any,TypeId};

//基本类型转换，用反射来判断
pub fn c_test1(){
    let a1:i32 = 100;
    let a2 = a1 as u32;
    let a3 = a2 as f64;
    let a4 = a3.to_string();
    let a5 = a1 as usize;
    assert_eq!(TypeId::of::<u32>()==a2.type_id(),true);
    assert_eq!(TypeId::of::<f64>()==a3.type_id(),true);
    assert_eq!(TypeId::of::<String>()==a4.type_id(),true);
    assert_eq!(TypeId::of::<usize>()==a5.type_id(),true);
    // assert_eq!(TypeId::of::<str>()==a4.type_id(),true); //str和&str都不行
    //String和&str转数字类型可以指定类型
    let a5:i32= String::from("123").parse().unwrap();
    let a6:f64= "123".parse().unwrap();
    assert_eq!(TypeId::of::<i32>()==a5.type_id(),true);
    assert_eq!(TypeId::of::<f64>()==a6.type_id(),true);
}

//测试所有基本类型在Default下的默认值
pub fn ct2(){
    println!("{:?}",PT::default());//PT { a: 0, b: 0, c: false, e: '\u{0}', f: 0.0 }
}

#[derive(Default,Debug)]
struct PT{
    a:i32,b:usize,c:bool,
    // d:str,//这个不能有
    e:char,f:f32
}