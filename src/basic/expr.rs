#[allow(dead_code)]
pub fn e1() {
    //path表达式
    let _some_constructor = Some::<i32>;
    let _push_integer = Vec::<i32>::push;
    let _slice_reverse = <[i32]>::reverse;
    //block表达式
    fn fn_call() {}
    let _: () = {
        fn_call();
    };
    let five: i32 = {
        fn_call();
        5
    };
    assert_eq!(5, five);
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
    //范围表达式
    1..2; // std::ops::Range, 1<=x<2
    3..; // std::ops::RangeFrom, 3<=x, 没有x>3的写法
    ..4; // std::ops::RangeTo,  x<4
    ..; // std::ops::RangeFull, 匹配所有
    5..=6; // std::ops::RangeInclusive, 5<=x<=6
    ..=7; // std::ops::RangeToInclusive, x<=7
}
