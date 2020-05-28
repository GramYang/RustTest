use std::fmt;
use std::error::Error;

//panic!使用测试
pub fn t1(){
    panic!();
    panic!("this is a terrible mistake!");
    panic!(4); // panic with the value of 4 to be collected elsewhere
    panic!("this is a {} {message}", "fancy", message = "message");
    let a=3;
    panic!("123 {}", a);//123 3
}

//Error trait使用
pub fn t2(){
    //source如果有的话返回底层source
    match get_super_error() {
        Err(e) => {
            println!("Error: {}", e);//Error: SuperError is here!
            println!("Caused by: {}", e.source().unwrap());//Caused by: SuperErrorSideKick is here!
        }
        _ => println!("No error"),
    }
}

#[derive(Debug)]
struct SuperError {
    side: SuperErrorSideKick,
}

impl fmt::Display for SuperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for SuperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.side)
    }
}

#[derive(Debug)]
struct SuperErrorSideKick;

impl fmt::Display for SuperErrorSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

//SuperErrorSideKick没有子类型了，因此Error只需要空实现
impl Error for SuperErrorSideKick {}

fn get_super_error() -> Result<(), SuperError> {
    Err(SuperError { side: SuperErrorSideKick })
}

//自定义error测试，文章：https://www.jianshu.com/p/74509cdaece1
//使用From修改上面t2的例子
pub fn t3() ->Result<(),CustomError>{
    let path = "asset/foo.txt";
    let v = read_file(path)?;
    let x = to_utf8(v.as_bytes())?;
    let u = to_u32(x)?;
    println!("{}",u);
    Ok(())
}

// 读取文件内容
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

// 转换为utf8内容
fn to_utf8(v: &[u8]) -> Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(v)
}

// 转化为u32数字
fn to_u32(v: &str) -> Result<u32, std::num::ParseIntError> {
    v.parse::<u32>()
}

#[derive(Debug)]
pub enum CustomError {
    ParseIntError(std::num::ParseIntError),
    Utf8Error(std::str::Utf8Error),
    IoError(std::io::Error),
}
impl std::error::Error for CustomError{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            CustomError::IoError(ref e) => Some(e),
            CustomError::Utf8Error(ref e) => Some(e),
            CustomError::ParseIntError(ref e) => Some(e),
        }
    }
}

impl fmt::Display for CustomError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CustomError::IoError(ref e) => e.fmt(f),
            CustomError::Utf8Error(ref e) => e.fmt(f),
            CustomError::ParseIntError(ref e) => e.fmt(f),
        }
    }
}

impl From<std::num::ParseIntError> for CustomError {
    fn from(s: std::num::ParseIntError) -> Self {
        CustomError::ParseIntError(s)
    }
}

impl From<std::io::Error> for CustomError {
    fn from(s: std::io::Error) -> Self {
        CustomError::IoError(s)
    }
}

impl From<std::str::Utf8Error> for CustomError {
    fn from(s: std::str::Utf8Error) -> Self {
        CustomError::Utf8Error(s)
    }
}