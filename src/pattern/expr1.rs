//一个奇怪的表达式：打印一个None数组，长度为10，其中else返回的数值会与cap相加作为数组的长度
pub fn e_test1(){
    let cap = 10;
    let o_arr : Vec<Option<i32>>= (0..cap + if cap == 0{1}else{0}).map(|_| None).collect();
    println!("{:?}",o_arr);
}