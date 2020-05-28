//Option测试，Option修改内部值不推荐你unwrap，而是map和and_then或者match
pub fn o_test1() {
    let x1: Option<u32> = Some(2);
    println!("{} {}", x1.is_some(), x1.is_none());//true false
    let x2 = Some("nmsl".to_string());
    //map不能作用与None，但是可以转变Some内置类型，与and_then对比
    let x3 = x2.as_ref().map(|s| s.len());//如果不加as_ref那么x2所有权就转移了
    println!("{} {}", x2.unwrap(), x3.unwrap());//nmsl 4
    let mut x4 = Some(2);//必须加mut
    match x4.as_mut() {//和上面的as_ref不同，这里可以修改解引用的值
        Some(v) => *v = 42,
        None => {},
    }
    println!("{}",x4.unwrap());//42
    let x5:Option<i32> = Some(2);
    let x6:Option<i32> = None;
    assert_eq!(x5.and(x6),None);//只要有一个是None就都是None
    let x7=Some(2);
    let x8=Some("foo");
    assert_eq!(x7.and(x8),Some("foo"));//后者覆盖前者的Some
    fn is_even(n:&i32)->bool{
        n%2==0
    }
    //测试filter
    assert_eq!(None.filter(is_even), None);
    assert_eq!(Some(3).filter(is_even), None);
    assert_eq!(Some(4).filter(is_even), Some(4));
    //测试get_or_insert。x9是None则将5插入转换为Some(5)；是Some(10)则不插入值；两种情况都会返回Some中值的可变引用
    let mut x9 = Some(10);
    {
        let x10 = x9.get_or_insert(5);
        assert_eq!(x10, &10);
        *x10 = 7;
    }
    assert_eq!(x9, Some(7));
    //测试take
    let mut x11 = Some(2);
    let x12 = x11.take();
    assert_eq!(x11, None);
    assert_eq!(x12, Some(2));
    //测试replace
    let mut x13 = Some(2);
    let old = x13.replace(5);
    assert_eq!(x13, Some(5));
    assert_eq!(old, Some(2));
    //flatten解除Option套娃，只能解除一层套娃
    let x: Option<Option<u32>> = Some(Some(6));
    assert_eq!(Some(6), x.flatten());
    let x: Option<Option<u32>> = Some(None);
    assert_eq!(None, x.flatten());
    let x: Option<Option<u32>> = None;
    assert_eq!(None, x.flatten());
    let x: Option<Option<Option<u32>>> = Some(Some(Some(6)));
    assert_eq!(Some(Some(6)), x.flatten());
    assert_eq!(Some(6), x.flatten().flatten());
    //and_then，None会返回None，Option内置类型不能转变，和map对比
    fn sq(x: u32) -> Option<u32> { Some(x * x) }
    fn nope(_: u32) -> Option<u32> { None }
    assert_eq!(Some(2).and_then(sq).and_then(sq), Some(16));
    assert_eq!(Some(2).and_then(sq).and_then(nope), None);
    assert_eq!(Some(2).and_then(nope).and_then(sq), None);
    assert_eq!(None.and_then(sq).and_then(sq), None);
}

//Result测试
pub fn r_test1(){
    //is_ok,is_err,ok,err
    let x1: Result<i32, &str> = Ok(-3);
    assert_eq!(x1.is_ok(), true);
    let x2: Result<i32, &str> = Err("Some error message");
    assert_eq!(x2.is_ok(), false);
    let x3: Result<i32, &str> = Ok(-3);
    assert_eq!(x3.is_err(), false);
    let x4: Result<i32, &str> = Err("Some error message");
    assert_eq!(x4.is_err(), true);
    let x5: Result<u32, &str> = Ok(2);
    assert_eq!(x5.ok(), Some(2));//Result和Option的转换
    let x6: Result<u32, &str> = Err("Nothing here");
    assert_eq!(x6.ok(), None);
    let x7: Result<u32, &str> = Ok(2);
    assert_eq!(x7.err(), None);
    let x8: Result<u32, &str> = Err("Nothing here");
    assert_eq!(x8.err(), Some("Nothing here"));
    //as_ref,as_mut同上
    let x9: Result<u32, &str> = Ok(2);
    assert_eq!(x9.as_ref(), Ok(&2));
    let x10: Result<u32, &str> = Err("Error");
    assert_eq!(x10.as_ref(), Err(&"Error"));
    fn mutate(r: &mut Result<i32, i32>) {
        match r.as_mut() {
            Ok(v) => *v = 42,
            Err(e) => *e = 0,
        }
    }
    let mut x11: Result<i32, i32> = Ok(2);
    mutate(&mut x11);
    assert_eq!(x11.unwrap(), 42);
    let mut x12: Result<i32, i32> = Err(13);
    mutate(&mut x12);
    assert_eq!(x12.unwrap_err(), 0);
    //map
    let line = "1\n2\n3\n4\n";
    for num in line.lines() {
        match num.parse::<i32>().map(|i| i * 2) {
            Ok(n) => println!("{}", n),
            Err(..) => {}
        }
    }//输出 2 4 6 8
}

//测试enum取参数
pub fn et3(){
    let s = State::A(10,20);
    match s{
        State::A{0:a,1:b} => println!("{} {}",a,b),
        _ => {},
    }
    let s1 = State::C{x:30,y:40};
    match s1{
        State::C{x,y} => println!("{} {}",x,y),
        _ => {},
    }
}

enum State{
    A(usize,usize),
    B(usize),
    C{x:usize,y:usize},
}