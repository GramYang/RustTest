//方法测试
#[allow(dead_code)]
pub fn s1() {
    //方法基本
    let mut b = Builder::new(100, String::from("蔡徐坤"));
    println!("{:?}", b.op1(1).op2(String::from("孙笑川")));
    //方法中所有权测试
    let mut b = Builder::new(123, "321".to_string());
    b.op3(); //123 321
    println!("{:?}", b); //Builder { a: 123, b: "321" }
    let b = Builder::new(1234, "4321".to_string());
    b.op4(); //1234 4321

    // println!("{:?}",b);//不能打印b，b已经被move了

    let b = Builder::new(12345, "54321".to_string());
    b.op5(); //Builder { a: 12345, b: "54321" }
    println!("{:?}", b); //Builder { a: 12345, b: "54321" }

    //引用参数
    let mut b = Builder::new(100, String::from("100"));
    let b1 = b.a;
    let b2 = b.b.clone();
    b.op6(&b1, &b2);
    println!("{:?}", b);
    //结构体
}

#[derive(Debug)]
struct Builder {
    a: i32,
    b: String,
}

#[allow(dead_code)]
impl Builder {
    fn new(x: i32, y: String) -> Builder {
        Builder { a: x, b: y }
    }
    fn op1(&mut self, x: i32) -> &mut Builder {
        self.a += x;
        self
    }
    fn op2(&mut self, y: String) -> &mut Builder {
        self.b += &y;
        self
    }
    fn op3(&mut self) {
        let a1 = self.a; //基本类型copy
                         // let b1 = self.b;//这里报错，can't move
        let b1 = self.b.clone(); //非基本类型可以这么写
        println!("{} {}", a1, b1);
    }
    fn op4(self) {
        let a1 = self.a;
        let b1 = self.b;
        println!("{} {}", a1, b1);
        // println!("{:?}",self);//报错，self被借用了
    }
    fn op5(&self) {
        let _a1 = self.a;
        let _b1 = &self.b;
        println!("{:?}", self);
    }
    fn op6(&mut self, x: &i32, y: &String) {
        self.a += x;
        self.b += y;
    }
}
