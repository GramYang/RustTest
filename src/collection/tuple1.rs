
//元组基本使用测试，元组不能被遍历
pub fn tp1(){
    let tuple = ("hello", 5, 'c');
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
}