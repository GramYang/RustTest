pub fn match_test1() {
    let a1=1;
    match a1{
        1=>println!("1"), //1
        2=>println!("2"),
        3=>println!("3"),
        _=>println!("anything"),
    }
    let a2=Some(5);
    let a3=10;
    match a2{
        Some(50)=>println!("50"),
        Some(a3)=>println!("y= {:?}",a3), //y= 5
        _=>println!("default,x={:?}", a2),
    }
    let a4=1;
    match a4{
        1|2=>println!("1 or 2"), //1 or 2
        3=>println!("3"),
        _=>println!("anything"),
    }
    let a5=5;
    match a5{
        1..=5 =>println!("1-5"), //1-5
        _=>println!("something else"),
    }
    let a6='j';
    match a6{
        'a'..='j'=>println!("a-j"), //a-j
        'k'..='z'=>println!("k-z"),
        _=>println!("something else"),
    }
}