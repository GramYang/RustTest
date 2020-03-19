//shadow测试
pub fn let_test1() {
    let x=1;
    let x =x+1;
    println!("{}",x) //2

}

//数据类型测试
pub fn let_test2() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}",guess); //42
    // let b:bool;
    // println!("{}",b); //b没有初始化，会报错
    let tup = (500,6.4,1);
    let (x,y,z)=tup;
    println!("{} {} {}",x,y,z); //500 6.4 1
    println!("{} {} {}",tup.0,tup.1,tup.2); //500 6.4 1
    let a=[1,2,3,4,5];
    let a1=[3;5];
    println!("{} {}",a[0], a1[0]); //1 3，只能这么访问，貌似不能直接整体输出
}