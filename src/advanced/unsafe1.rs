
//裸指针
pub fn p_test1(){
    let mut num=5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let r3 = unsafe{&*r2};
    let r4 = unsafe {&*r1};
    unsafe{
        println!("{:p} {:p} {:p} {:p}",r1,r2,r3,r4); //四个地址一致
    }
    let mut a = Box::new(num);
    let raw = &mut *a; //这种写法不用as
    println!("{:p}",raw); //地址和上面不一致
}

//unsafe的函数和方法
pub fn f_test1(){
    let mut v=vec![1,2,3,4,5,6];
    let r=&mut v[..];
    let (a,b)=r.split_at_mut(3);
    assert_eq!(a,&mut[1,2,3]);
    assert_eq!(b,&mut[4,5,6]);
}

extern "C"{
    fn abs(input:i32)->i32;
}

//extern使用
pub fn e_test1() {
    unsafe{
        println!("{}", abs(-3)); //3，可以直接用！！
    }
}

static HELLO_WORLD:&str="hello, world!";
static mut COUNTER:u32=0;

//可变和不可变静态变量
pub fn s_test1() {
    println!("{}", HELLO_WORLD);
    unsafe{
        COUNTER+=1;
        println!("{}", COUNTER);
    }
}