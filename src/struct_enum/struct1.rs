
//方法写法测试
pub fn s_test1(){
    //Builder模式
    let b = Builder::new(100,String::from("nmsl"));
    println!("{:?}",b.op1(1).op2(String::from("1")));//Builder { a: 101, b: "nmsl1" }
    //所有权测试
    let mut b = Builder::new(123, "321".to_string());
    b.op4();
    println!("{:?}",b);//不变
    let b = Builder::new(1234, "4321".to_string());
    b.op5();
    //Builder模式+Option
    let b = Builder1::new(200,String::from("nmsl"));
    println!("{:?}",b.op1(1).op2(String::from("1")));//Builder1 { a: Some(201), b: Some("nmsl1") }
    //go指针方法
    let mut b = Builder1::new(300,String::from("omfg"));
    &b.op3(2,String::from("1"));//调用方法，b和&b都一样
    println!("{:?}",b);//Builder1 { a: Some(302), b: Some("omfg1") }
    //引用参数
    let mut b = Builder::new(350, String::from("田所浩二"));
    let b1 = b.a.clone();
    let b2 = b.b.clone();
    b.op3(&b1,&b2);
    println!("{:?}",b);//Builder { a: 700, b: "田所浩二田所浩二" }
    //引用参数+Option，实例使用实例的值只能这么写。
    let mut b = Builder1::new(400, String::from("wdnmd"));
    b.op4(b.a.clone().as_ref().unwrap(), b.b.clone().as_ref().unwrap());//必须要加clone，虽然clone和as_ref都只使用引用
    println!("{:?}",b);//Builder1 { a: Some(800), b: Some("wdnmdwdnmd") }
    //引用参数+Option+Vec+函数指针
    fn p(x:i32) ->i32{
        x*2
    };
    fn p1(x:i32)->i32{
        x*3
    }
    fn p2(x:i32)->i32{
        x*4
    }
    fn p3(x:i32)->i32{
        x*5
    }
    let mut b = Builder2::new(p);
    let mut v1:Vec<Ptr> = vec![];
    v1.push(p1);
    v1.push(p2);
    b.op1(&v1);
    b.b = Some(p3);
    println!("{}",b.op2(2)(3));//12
    println!("{}",b.b.unwrap()(3));//15
    println!("{:?}",b);//Builder2 { a: Some([0x406b00, 0x406b40, 0x406b80]) }
}

#[derive(Debug)]
struct Builder{
    a:i32,
    b:String,
}

impl Builder{
    fn new(x:i32,y:String) -> Builder{
        Builder{a:x,b:y}
    }

    fn op1(mut self, x:i32) -> Builder {
        self.a+=x;
        return self;
    }

    fn op2(mut self, y:String) -> Builder{
        self.b += &y;
        return self;
    }

    //引用参数
    fn op3(&mut self, x:&i32, y:&String){
        self.a+=*x;
        self.b += y;
    }

    //所有权测试1
    fn op4(&mut self){
        let a1 = self.a;//基本类型可以move
        // let b1 = self.b;//这里报错，can't move
        let b1 = self.b.clone();//非基本类型可以这么写
        println!("{} {}",a1,b1);//123 321
    }

    //所有权测试2
    fn op5(mut self) {
        let a1 = self.a;
        let b1 = self.b;
        println!("{} {}",a1,b1);//1234 4321
        // println!("{:?}",self);//报错，self被借用了
    }
}

#[derive(Debug)]
struct Builder1{
    a:Option<i32>,
    b:Option<String>,
}

impl Builder1{
    fn new(x:i32,y:String) -> Builder1{
        Builder1{a:Some(x),b:Some(y)}
    }

    fn op1(mut self, x:i32) -> Builder1 {
        self.a.as_mut().map(|mut s|*s+=x);
        return self;
    }

    fn op2(mut self, y:String) -> Builder1{
        self.b.as_mut().map(|s|s.push_str(&y));
        return self;
    }

    //go的指针方法
    fn op3(&mut self, x:i32, y:String){
        self.a.as_mut().map(|mut s|*s+=x);
        self.b.as_mut().map(|s|s.push_str(&y));
    }

    //引用参数
    fn op4(&mut self, x:&i32, y:&String){
        self.a.as_mut().map(|mut s| *s+=*x);
        self.b.as_mut().map(|s|s.push_str(y));
    }
}

#[derive(Debug)]
struct Builder2{
    a:Option<Vec<Ptr>>,
    b:Option<Ptr>,
}

type Ptr = fn(x:i32) ->i32;

impl Builder2{
    fn new(x:Ptr)->Self{//非trait和implement使用Self也行！
        let mut v:Vec<Ptr> = Vec::new();
        v.push(x);
        Builder2{
            a:Some(v),
            b:None,
        }
    }

    //引用参数+Option+Vec+函数指针
    fn op1(&mut self, ps:&Vec<Ptr>){
        self.a.as_mut().map(|s| s.extend_from_slice(ps));
    }

    //取函数指针
    fn op2(&mut self, index:usize)->&Ptr{
        let p = self.a.as_ref().unwrap().get(index).unwrap();
        return p;
    }
}


