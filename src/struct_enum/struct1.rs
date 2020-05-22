use std::cell::RefCell;
use std::rc::Rc;
use std::convert::TryInto;

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
    fn p2(mut x:i32)->i32{//添加mut后p2仍然属于Ptr，但是加&mut就不行了
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

//测试：方法在传递引用时，擅自给引用添加mut会修改原来的值吗？需要。
//方法的&参数可以接收&mut参数，反过来不行。
//方法不是引用的话mut无限制。
pub fn s_t2(){
    let mut b = Builder3::new(String::from("114"));
    let mut s1 = String::from("51");
    b.op1(&mut s1);//Builder3 { s: "114514" }
    let s2 = String::from("1919");
    b.op2(s2);//1919810
}

#[derive(Debug)]
struct Builder3{
    s:String
}

impl Builder3{
    fn new(s:String) ->Self{
        Builder3{s}
    }

    fn op1(&mut self, x: &mut String){
        x.push('4');
        self.s.push_str(x);
        println!("{:?}",self);
    }

    fn op2(&self, mut x:String){
        x.push_str("810");
        println!("{}",x);
    }

    fn op3(&self){
    }

    fn op4(&mut self){
    }
}

//测试智能指针层层包裹下的方法调用
pub fn s_t3(){
    //RefCell内部值的方法调用
    let a = Builder3{s:"114".to_string()};
    let b = Builder3{s:"514".to_string()};
    let c = Rc::new(RefCell::new(b));
    a.op3();
    c.borrow().op3();
    let mut a1 = Builder3{s:"114".to_string()};
    let mut b1 = Builder3{s:"514".to_string()};
    let c1 = Rc::new(RefCell::new(b1));
    a1.op4();
    c1.borrow_mut().op4();
}