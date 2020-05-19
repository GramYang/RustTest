//一个奇怪的表达式：打印一个None数组，长度为10，其中else返回的数值会与cap相加作为数组的长度
pub fn e_test1(){
    let cap = 10;
    let o_arr : Vec<Option<i32>>= (0..cap + if cap == 0{1}else{0}).map(|_| None).collect();
    println!("{:?}",o_arr);
}

//各种表达式测试
pub fn e_test2(){
    //path表达式
    let some_constructor = Some::<i32>;
    let push_integer = Vec::<i32>::push;
    let slice_reverse = <[i32]>::reverse;
    //block表达式
    fn fn_call(){}
    let _: () = {
        fn_call();
    };
    let five: i32 = {
        fn_call();
        5
    };
    assert_eq!(5, five);
    struct Struct;
    impl Struct {
        fn consume_self(self) {}
        fn borrow_self(&self) {}
    }
    fn move_by_block_expression() {
        let s = Struct;
        // Move the value out of `s` in the block expression.
        (&{s}).borrow_self();
        // Fails to execute because `s` is moved out of.
        // s.consume_self();
    }
    //操作符表达式
    let x = 6;
    assert_eq!(-x, -6);
    assert_eq!(!x, -7);
    assert_eq!(true, !false);
    assert_eq!(3 + 6, 9);
    assert_eq!(5.5 - 1.25, 4.25);
    assert_eq!(-5 * 14, -70);
    assert_eq!(14 / 3, 4);
    assert_eq!(100 % 7, 2);
    assert_eq!(0b1010 & 0b1100, 0b1000);
    assert_eq!(0b1010 | 0b1100, 0b1110);
    assert_eq!(0b1010 ^ 0b1100, 0b110);
    assert_eq!(13 << 3, 104);
    assert_eq!(-10 >> 2, -3);
    assert!(123 == 123);
    assert!(23 != -12);
    assert!(12.5 > 12.2);
    assert!([1, 2, 3] < [1, 3, 4]);
    assert!('A' <= 'B');
    assert!("World" >= "Hello");
    //数组表达式
    ([1, 2, 3, 4])[2];        // Evaluates to 3
    let b = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    b[1][2];                  // multidimensional array indexing
    // let x = (["a", "b"])[10]; // warning: index out of bounds
    let n = 10;
    // let y = (["a", "b"])[n];  // panics
    let arr = ["a", "b"];
    // arr[10];                  // warning: index out of bounds
    //元组表达式
    // let pair = (1, 2);
    // assert_eq!(pair.1, 2);
    // let unit_x = Point(1.0, 0.0);
    // assert_eq!(unit_x.0, 1.0);
    //结构体表达式
    struct Color(u8, u8, u8);
    let c1 = Color(0, 0, 0);  // Typical way of creating a tuple struct.
    let c2 = Color{0: 255, 1: 127, 2: 0};  // Specifying fields by index.
    let c3 = Color{1: 0, ..c2};  // Fill out all other fields using a base struct.
    // Point3d { x: x, y: y_value, z: z };
    // Point3d { x, y: y_value, z };//x和z简写了
    struct Position(i32, i32, i32);
    let c1= Position(0, 0, 0);  // Typical way of creating a tuple struct.
    let c = Position;  // `c` is a function that takes 3 arguments.
    let pos = c(8, 6, 7);  // Creates a `Position` value.
    struct Gamma;
    let a = Gamma;  // Gamma unit value.
    let b = Gamma{};  // Exact same value as `a`.
    //枚举表达式
    // let q = Message::Quit;
    // let w = Message::WriteString("Some string".to_string());
    // let m = Message::Move { x: 50, y: 200 };
    //call表达式
    // let three: i32 = add(1i32, 2i32);
    let name: &'static str = (|| "Rust")();
    trait Pretty {
        fn print(&self);
    }
    trait Ugly {
        fn print(&self);
    }
    struct Foo;
    impl Pretty for Foo {
        fn print(&self) {}
    }
    struct Bar;
    impl Pretty for Bar {
        fn print(&self) {}
    }
    impl Ugly for Bar{
        fn print(&self) {}
    }
    let f = Foo;
    let b = Bar;
    // we can do this because we only have one item called `print` for `Foo`s
    f.print();
    // more explicit, and, in the case of `Foo`, not necessary
    Foo::print(&f);
    // if you're not into the whole brevity thing
    <Foo as Pretty>::print(&f);
    // b.print(); // Error: multiple 'print' found
    // Bar::print(&b); // Still an error: multiple `print` found
    // necessary because of in-scope items defining `print`
    <Bar as Pretty>::print(&b);
    //域访问表达式
    struct A { f1: String, f2: String, f3: String }
    let mut x = A{f1:"s".to_string(),f2:"s".to_string(),f3:"s".to_string()};
    let a: &mut String = &mut x.f1; // x.f1 borrowed mutably
    let b: &String = &x.f2;         // x.f2 borrowed immutably
    let c: &String = &x.f2;         // Can borrow again
    // let d: String = x.f3;           // Move out of x.f3
    //闭包表达式
    fn ten_times<F>(f: F) where F: Fn(i32) {
        for index in 0..10 {
            f(index);
        }
    }
    ten_times(|j| println!("hello, {}", j));
    // With type annotations
    ten_times(|j: i32| -> () { println!("hello, {}", j) });
    let word = "konnichiwa".to_owned();
    ten_times(move |j| println!("{}, {}", word, j));
    //范围表达式
    1..2;   // std::ops::Range, 1<=x<2
    3..;    // std::ops::RangeFrom, 3<=x, 没有x>3的写法
    ..4;    // std::ops::RangeTo,  x<4
    ..;     // std::ops::RangeFull, 匹配所有
    5..=6;  // std::ops::RangeInclusive, 5<=x<=6
    ..=7;   // std::ops::RangeToInclusive, x<=7
    //if let表达式，可以和match互换
    let x = Some(3);
    let a = if let Some(1) = x {
        1
    } else if x == Some(2) {
        2
    } else if let Some(y) = x {
        y
    } else {
        -1
    };
    assert_eq!(a, 3);
    enum E {
        X(u8),
        Y(u8),
        Z(u8),
    }
    let v = E::Y(12);
    if let E::X(n) | E::Y(n) = v {
        assert_eq!(n, 12);
    }
}