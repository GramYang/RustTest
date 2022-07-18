use std::any::{Any, TypeId};

//变量基础
#[allow(dead_code)]
pub fn v1() {
    //shadow隐藏
    let x = 1;
    let _x = x + 1;
    let _x = "1";
    //常量
    const _B1: u32 = 1 << 0;
    const _B2: u32 = 1 << 1;
    const _BS: [u32; 2] = [_B1, _B2];
    //字符串常量
    const _S1: &'static str = "bitstring";
    //实例常量
    struct BitsNStrings<'a> {
        mybits: [u32; 2],
        mystring: &'a str,
    }
    const _BITS: BitsNStrings<'static> = BitsNStrings {
        mybits: _BS,
        mystring: _S1,
    };
    //实例常量指定drop，实例不能带生命周期
    struct TypeWithDestructor(i32);
    impl Drop for TypeWithDestructor {
        fn drop(&mut self) {
            println!("Dropped. Held {}.", self.0);
        }
    }
    const ZERO_WITH_DESTRUCTOR: TypeWithDestructor = TypeWithDestructor(0);
    fn create_and_drop_zero_with_destructor() {
        let _x = ZERO_WITH_DESTRUCTOR;
        // x gets dropped at end of function, calling drop.
        // prints "Dropped. Held 0.".
    }
    create_and_drop_zero_with_destructor()
}

//&和*的使用
#[allow(dead_code)]
pub fn v2() {
    let a = 5;
    let b = &a;
    let c = &a; //可以有多个不可变引用，不能和可变引用共存
    let d = c; //&的赋值不会move，会copy
    println!("{:p} {:p} {:p} {:p}", &a, &&a, &&&a, &&&&a); //四个不同地址
    println!("{:p} {:p} {:p}", b, c, d); //三个相同地址
    let mut a1 = 6;
    {
        //必须写在b1上面
        let _b2 = &mut a1;
    }
    let b1 = &mut a1; //一个区域内只能有一个可变引用
    let c1 = &*b1; //*会消除mut
    let c2 = &*b1; //通过*消除mut可以获得多个不可变引用
    println!("{:p} {:p} {:p}", b1, c1, c2); //三个地址相同
    let s1 = String::from("123");
    let s2 = &s1;
    let s3 = s2; //引用和基本类型一样，不会move只会copy
    println!("{:p} {:p} {:p}", &s1, s2, s3);
}

//类型转换
#[allow(dead_code)]
pub fn v3() {
    let a1 = 100;
    let a2 = a1 as u32;
    let a3 = a2 as f64;
    let a4 = a3.to_string();
    let a5 = a1 as usize;
    assert_eq!(TypeId::of::<u32>() == a2.type_id(), true);
    assert_eq!(TypeId::of::<f64>() == a3.type_id(), true);
    assert_eq!(TypeId::of::<String>() == a4.type_id(), true);
    assert_eq!(TypeId::of::<usize>() == a5.type_id(), true);
    // assert_eq!(TypeId::of::<str>()==a4.type_id(),true); //str和&str都不行
    //String和&str转数字类型可以指定类型
    let a5: i32 = String::from("123").parse().unwrap();
    let a6: f64 = "123".parse().unwrap();
    assert_eq!(TypeId::of::<i32>() == a5.type_id(), true);
    assert_eq!(TypeId::of::<f64>() == a6.type_id(), true);
    //基本类型默认值
    ct1();
}

#[allow(dead_code)]
#[derive(Default, Debug)]
struct PT {
    a: i32,
    b: usize,
    c: bool,
    // d:str,//这个不能有
    e: char,
    f: f32,
}

//测试所有基本类型在Default下的默认值
fn ct1() {
    println!("{:?}", PT::default()); //PT { a: 0, b: 0, c: false, e: '\u{0}', f: 0.0 }
}

//特殊类型
#[allow(dead_code)]
pub fn v4() {
    //单元类型+单元结构体，大小都是0
    assert_eq!(std::mem::size_of::<()>(), 0); //单元类型
    assert_eq!(std::mem::size_of::<Foo>(), 0); //单元结构体
    assert_eq!(std::mem::size_of::<Baz>(), 0); //复合结构体
    assert_eq!(std::mem::size_of::<Void>(), 0); //单元枚举体
    assert_eq!(std::mem::size_of::<[(); 10]>(), 0); //长度为10的单元类型数组
}

enum Void {}
struct Foo;

#[allow(dead_code)]
struct Baz {
    foo: Foo,
    quz: (),      //单元类型
    baz: [u8; 0], //长度0的数组
}
