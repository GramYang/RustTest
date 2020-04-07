use std::fs::File;
use std::io::{self,Read,ErrorKind};
use std::error::Error;

enum IpAddrKind{
    V4,
    V6(u8,u8,u8,u8),
}

enum Message{
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum Count {
    Zero,One,Two,Three,Four
}

fn values_in_cents(coin:Coin)->u8{
    match coin{
        Coin::Penny=>1,
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quarter(state)=>{
            println!("{:?}",state); //配合#[derive(Debug)]才能输出
            25
        },
    }
}

struct IpAddr{
    kind:IpAddrKind,
    address:String,
}

impl Message {
    fn call(&self){

    }
}

//枚举使用
pub fn enum_test1() {
    let _=IpAddr{
        kind:IpAddrKind::V4,
        address:String::from("127.0.0.1"),
    };
    let _=IpAddr{
        kind:IpAddrKind::V6(127,0,0,1),
        address:String::from("::1"),
    };
    let m=Message::Write(String::from("hello"));
    m.call();
    let a1 = Coin::Quarter(UsState::Alabama);
    println!("{}",values_in_cents(a1)); //25
    const NUM:u32=Count::Four as u32;
    println!("{}", NUM); //4
    //将枚举的项转换成u32，这是可行的！创建一个enum，里面的项默认就是u32
}

//Option、match、if let
pub fn enum_test2() {
    let five=Some(5);
    let _=plus_one(five);
    let _=plus_one(None);
    let a1=Some(0u8);
    println!("{}",five.unwrap()); //5，通过unwrap()来读取Some里面的值
    let five = match five{ //另外一种从枚举中取出值的方式
        Some(5)=>5,
        _=>panic!("wrong!"),
    };
    println!("{}",five);
    match a1{
        Some(3)=>println!("three"),
        _=>(),
    }
    if let Some(0u8) = a1{
        println!("0u8") //输出
    }
    if a1==Some(0u8){
        println!("0u8") //输出
    }
}

fn plus_one(x:Option<i32>)->Option<i32>{
    match x{
        None=>None,
        Some(i)=>Some(i+1),
    }
}

//Result、错误匹配、unwrap、expect
pub fn enum_test3() {
    //错误匹配
    let f1=File::open("hello.txt");
    let _=match f1{
        Ok(file)=>file,
        Err(err)=>match err.kind(){
            ErrorKind::NotFound=>match File::create("hello.txt"){
                Ok(fc)=>fc,
                Err(e)=>panic!("Problem creating the file: {:?}",e),
            },
            other_err=>panic!("Problem opening the file: {:?}",other_err),
        },
    };
    //unwrap和expect其实就是对上面的封装
    let _=File::open("hello.txt").unwrap();
    let _=File::open("hello.txt").expect("wrong!");
    let _=read();//返回Result
    let _=read1();//用?进行简写
    let _=main1();//模拟main函数
}

fn read()->Result<String,io::Error>{
    let f=File::open("hello.txt");
    let mut f=match f{
        Ok(file)=>file,
        Err(e)=>return Err(e),
    };
    let mut s=String::new();
    match f.read_to_string(&mut s){
        Ok(_)=>Ok(s),
        Err(e)=>Err(e),
    }
}

fn read1()->Result<String,io::Error>{
    let mut s=String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main1()->Result<(),Box<dyn Error>>{
    let f=File::open("hello.txt")?;
    Ok(())
}