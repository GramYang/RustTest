
//函数
#[allow(dead_code)]
pub fn f1(){
    //常规
    let a = [1, 2, 3, 4, 8, 9];
    println!("{} {}", find(7, &a),find(8, &a));
    //dyn和impl
    let b = Boob{a:100};
    let _b1 = impl_test(b);
    // println!("{:?}",b1);//报错，impl ImplTest不能打印
    trait Trait {}
    impl Trait for i32 {}
    fn function2() -> Box<dyn Trait> {
        Box::new(1)
    }//Box内使用trait必须要加dyn来表示Trait是一个trait
}

fn find(n: i32, a: &[i32]) -> bool {
    for i in a {
        if *i == n {
            return true;
        }
    }
    false
}

struct Boob{
    a:i32,
}

trait ImplTest{
    fn it(&self);
}

impl ImplTest for Boob{
    fn it(&self){
        println!("{}", self.a);
    }
}

fn impl_test(x:impl ImplTest) -> impl ImplTest{
    x.it();
    return x;
}