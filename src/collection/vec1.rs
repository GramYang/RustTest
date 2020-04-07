//vector使用
pub fn vec_test1() {
    //创建一个空vector
    // let _ =Vec::new();
    //用vec宏创建非空vector
    let v=vec![1,2,3,4,5];
    //访问vec的值的两种方法
    let third=&v[2];
    println!("{}",third);
    match v.get(2){
        Some(third)=>println!("{}",third),
        None=>println!("木有"),
    }
    //遍历修改可变vec
    let mut v=vec![100,32,57];
    for i in &mut v{
        *i+=50;
    }
}