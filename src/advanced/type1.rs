
//type使用
pub fn t_test1(){
    type Kilometers=i32;
    let x=5;
    let y:Kilometers=5;
    println!("{}",x+y); //10
    type Thunk=Box<dyn Fn()+Send+'static>;
    let f:Thunk=Box::new(||println!("hi"));
    type Result1<T>=Result<T, std::io::Error>;
}