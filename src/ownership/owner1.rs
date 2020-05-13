//所有权和函数
pub fn owner_test1() {
    let s =String::from("hello");
    takes_ownership(s);//字符串s移动到函数里，函数执行完s不再有效
    let x=5;
    makes_copy(x); //x虽然也是移动到函数里，但是仍然有效，因为x是Copy的
}

fn takes_ownership(s:String) {
    println!("{}",s);
}

fn makes_copy(i:i32){
    println!("{}",i)
}

//返回值与作用域
pub fn owner_test2() {
    let _=gives_ownership(); //函数返回值移给s1
    let s2=String::from("hello");
    let _=takes_and_gives_back(s2); //s2被移动到函数中
}

fn gives_ownership() ->String{
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(s:String)->String{
    s
}

//引用和借用
pub fn owner_test3() {
    let s1=String::from("hello");
    let len=calculate_length(&s1); //不可变引用
    println!("{} {}",s1,len); //hello 5
    // change1(&s1);//&默认是不可变引用
    let mut s2=String::from("world");
    change2(&mut s2); //可变引用
    println!("{}",s2); //world,world
}

fn calculate_length(s:&String)->usize{
    s.len()
}

// fn change1(s:&String){
//     s.push_str(",world");
// }

fn change2(s:&mut String){
    s.push_str(",world");
}

//可变引用只能有一个
pub fn owner_test4() {
    let mut s=String::from("hello");
    let r1=&mut s;
    // let r2 =&mut s; //报错，s同一个作用域只能有一个可变引用
    println!("{}",r1);
    let mut b=String::from("world");
    {
        let _=&mut b;
    }
    let r3=&mut b;
    // let r4=&b; //b不能同时拥有不变引用和可变引用
    // println!("{}, {}",r3,r4);
    //悬垂引用
    // let _=dangle(); //悬垂引用
}

// fn dangle()->&String{
//     let s=String::from("hello");
//     &s
// }

//slice
pub fn owner_test5() {
    let s1=String::from("hello world");
    let slice=&s1[..2];
    let s2 = "Hello, world";
    //下面三种写法都可以
    let s3=first_word(&s1[..]);
    let s4=first_word(&s2[..]);
    let s5 =first_word(s2);
    println!("{} {} {}",s3,s4,s5);
}

fn first_word(s:&str)->&str {
    let bytes=s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item==b' '{
            return &s[0..i];
        }
    }
    &s[..]
}