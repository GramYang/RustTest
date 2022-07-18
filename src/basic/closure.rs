//闭包
#[allow(dead_code)]
pub fn c1() {
    //普通闭包
    let a = |x| x + 1;
    println!("{}", a(1)); //2，这个时候闭包c中参数x的类型固定为i32，用其他的类型将会报错
                          //泛型函数闭包
    let mut c = Cacher::new(|a| a);
    println!("{} {}", c.value(1), c.value(2));
    //闭包捕获环境
    let x = 4;
    let equal_to_x = |z| z == x;
    println!("{}", equal_to_x(x)); //该闭包不可变的借用了x
    println!("{}", x); //4，还有效
    let x1 = vec![1, 2, 3];
    let equal_to_x1 = move |z| z == x1;
    println!("{}", equal_to_x1(vec![1, 2, 3])); //true
                                                // println!("{:?}",x1); //报错，x1被借用，如果x1是基础类型则不会

    //Fn
    fn call_with_one<F>(func: F) -> usize
    where
        F: Fn(usize) -> usize,
    {
        func(1)
    }
    let double = |x| x * 2;
    assert_eq!(call_with_one(double), 2);
    //FnMut
    let mut x = 5;
    {
        let mut square_x = || x *= x;
        square_x();
    }
    assert_eq!(x, 25);
    fn do_twice<F>(mut func: F)
    where
        F: FnMut(),
    {
        func();
        func();
    }
    let mut x: usize = 1;
    {
        let add_two_to_x = || x += 2;
        do_twice(add_two_to_x);
    }
    assert_eq!(x, 5);
    //FnOnce
    fn consume_with_relish<F>(func: F)
    where
        F: FnOnce() -> i32,
    {
        println!("Consumed: {}", func());
        println!("Delicious!");
    }
    let x = 1;
    let consume_and_return_x = move || x;
    consume_with_relish(consume_and_return_x);
    consume_with_relish(consume_and_return_x); //如果x是String这里就报错了
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    //Fn是闭包函数类型
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(cal: T) -> Cacher<T> {
        Cacher {
            calculation: cal,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
