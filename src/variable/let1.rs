use std::any::{Any,TypeId};
use std::net::SocketAddr;

//shadow测试
pub fn let_test1() {
    let x=1;
    let x =x+1;
    println!("{}",x) //2

}

//数据类型测试
pub fn let_test2() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}",guess); //42
    let addr :SocketAddr= "127.0.0.1:8090".parse().unwrap();
    println!("{}", addr); //127.0.0.1:8090
    // let b:bool;
    // println!("{}",b); //b没有初始化，会报错
    let tup = (500,6.4,1);
    let (x,y,z)=tup;
    println!("{} {} {}",x,y,z); //500 6.4 1
    println!("{} {} {}",tup.0,tup.1,tup.2); //500 6.4 1
    let a=[1,2,3,4,5];
    let a1=[0;5];
    println!("{} {}",a[0], a1[0]); //1 3，只能这么访问，貌似不能直接整体输出
    //用反射判断一下a1中元素的类型
    println!("{}", TypeId::of::<i32>()==a1[0].type_id()); //true
}

//常量和静态变量
pub fn let_test3(){
    //一般常量
    const BIT1:u32=1<<0;
    const BIT2:u32=1<<1;
    const BITS: [u32; 2] = [BIT1, BIT2];
    //String常量
    const STRING: &'static str = "bitstring";
    struct BitsNStrings<'a> {
        mybits: [u32; 2],
        mystring: &'a str,
    }
    //常量实例
    const BITS_N_STRINGS: BitsNStrings<'static> = BitsNStrings {
        mybits: BITS,
        mystring: STRING,
    };
    //常量实例也是可以指定drop的
    struct TypeWithDestructor(i32);

    impl Drop for TypeWithDestructor {
        fn drop(&mut self) {
            println!("Dropped. Held {}.", self.0);
        }
    }

    const ZERO_WITH_DESTRUCTOR: TypeWithDestructor = TypeWithDestructor(0);

    fn create_and_drop_zero_with_destructor() {
        let x = ZERO_WITH_DESTRUCTOR;
        // x gets dropped at end of function, calling drop.
        // prints "Dropped. Held 0.".
    }

    //无名常量
    const _: () =  { struct _SameNameTwice; };
    // OK although it is the same name as above:
    const _: () =  { struct _SameNameTwice; };
    //宏可以多次发出未命名常量
    macro_rules! m {
    ($item: item) => { $item $item }
    }

    m!(const _: () = (););
    // This expands to:
    // const _: () = ();
    // const _: () = ();

    //对可变静态变量的使用必须套unsafe
    static mut LEVELS: u32 = 0;
    //这违反了没有共享状态的概念，并且这并不能在内部防止竞争，因此这个函数是“不安全的”
    unsafe fn bump_levels_unsafe1() -> u32 {
        let ret = LEVELS;
        LEVELS += 1;
        return ret;
    }
    //假设我们有一个返回旧值的atomic_add函数，这个函数是“安全的”，但是返回值的含义可能不是调用者所期望的，
    //所以它仍然被标记为“不安全的”
    // unsafe fn bump_levels_unsafe2() -> u32 {
    //     return atomic_add(&mut LEVELS, 1);
    // }
}