//字符串的使用
pub fn str_test1() {
    //创建一个空String
    let mut s=String::new();
    s.push_str("bar");
    s.push('y');
    println!("{}",s);
    //&str切换String
    let data="initial contents";
    let _=data.to_string();
    let _="initial contents".to_string();
    //String切换&str
    let a = String::from("1234");
    let _= &a[..];
    //&&str类型
    let b = &"asd";
    //拼接
    let s1=String::from("tic");
    let s2=String::from("tac");
    let s3=String::from("toe");
    let s4=s1+"-"+&s2+"-"+&s3;
    println!("{}",s4);
    // let h=s4[0]; //错误，String不支持内部索引
}