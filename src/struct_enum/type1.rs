
//测试自定义类型是否可以实现trait和impl？不行。这一点和go不一样。
pub fn t_t1(){
    // let b = Beans::new(1,String::from("a"),String::from("aa"));
}

// trait Pick{
//     fn op1(&self);
// }
//
// #[derive(Debug)]
// struct Bean{
//     a:i32,
//     b:String,
//     c:Option<String>,
// }
//
// type Beans = Vec<Bean>;
//
// impl Beans{
//     fn new(x:i32,y:String,z:String) ->Self{
//         let mut v = vec![];
//         v.push(Bean{a:x,b:y,c:Some(z)});
//         v
//     }
// }