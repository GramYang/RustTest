use std::ops::Add;
use std::ptr::NonNull;
use std::cell::Cell;
use serde::export::PhantomData;
use std::rc::Rc;

//裸指针
pub fn p_test1(){
    let mut num=5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;//这里跳出了可变引用和不可变引用不能同时存在的限制
    let r21:*mut _=&mut num;//把可变裸指针的定义放到左边也行
    //另一种获取可变裸指针的方法，但是其指针和r21以及其他的指针都不同，说明被分配到了堆上。
    let r22=Box::into_raw(Box::new(num));
    let r3 = unsafe{&*r2};
    let r4 = unsafe {&*r1};
    let r5 = unsafe{*r1};//这里move了，由于是基本类型所以是copy
    let r6 = &r1;//裸指针的引用
    let r7 = *r6;//获取该裸指针
    let r8 = unsafe{**r6};//获取裸指针指向值
    struct Bag(i32);
    let mut b = Bag{0:10};
    let b1 = &b as *const Bag;
    let b2 = &mut b as *mut Bag;
    let b3 = unsafe{&*b2};
    let b4 = unsafe{&*b3};
    // let b5 = unsafe{*b1};//这里move了，因为不是基本类型，所以报错
    unsafe{
        println!("{:p} {:p} {:p} {:p} {:p} {:p} {:p}",r1,r2,r21,r22,r3,r4,&r5);
        //0xcffaac 0xcffaac 0xcffaac 0x1324760 0xcffaac 0xcffaac 0xcffad4
        println!("{:p} {:p} {:p} {:p}", b1,b2,b3,b4);
        //0xd0fa04 0xd0fa04 0xd0fa04 0xd0fa04
    }
    let mut a = Box::new(num);
    let raw = &mut *a; //这种写法不用as
    println!("{:p}",raw); //0xea7c10
    let raw_into = Box::into_raw(a);
    println!("{:p}",raw_into);//0xea7c10，看来Box是把num整体放到了堆上面
    let a = [1, 2, 3];
    println!("{:p} {:p} {:p}",a.as_ptr(),&a as *const [i32],&a as *const [i32] as *const i32);//三者相等
    //ptr::null_mut()原理测试
    let a=std::ptr::null_mut::<i32>();
    println!("{:p}",a);//0x0，这是一个空指针，所以*a会报错
}

//裸指针操作测试
pub fn p2(){
    //add计算一个指针的间距，参数count是T的字节数倍数，就是向后移动指针
    let s: &str = "123";
    let ptr: *const u8 = s.as_ptr();
    unsafe {
        println!("{}", *ptr.add(1) as char);//2
        println!("{}", *ptr.add(2) as char);//3
    }
    //offset同add
    let s: &str = "nmsl";
    let ptr: *const u8 = s.as_ptr();
    unsafe {
        println!("{}", *ptr.offset(1) as char);//m
        println!("{}", *ptr.offset(2) as char);//s
    }
    //align_offset填充对齐间距
    fn foo(n: usize) {
        use std::mem::align_of;
        unsafe {
            let x = [5u8, 6u8, 7u8, 8u8, 9u8];
            let ptr = &x[n] as *const u8;
            let offset = ptr.align_offset(align_of::<u16>());
            println!("{}",offset);
            if offset < x.len() - n - 1 {
                let u16_ptr = ptr.add(offset) as *const u16;
                assert_ne!(*u16_ptr, 500);
            } else {
                // while the pointer can be aligned via `offset`, it would point
                // outside the allocation
            }
        }
    }
    foo(3);//0
    //as_mut从*mut返回&mut
    let mut s = [1, 2, 3];
    let mut ptr: *mut u32 = s.as_mut_ptr();
    let first_value = unsafe {
        ptr = ptr.add(2);
        ptr.as_mut().unwrap()
    };
    *first_value = 4;
    println!("{:?}", s); // It'll print: "[1, 2, 4]".
    let mut s = [1, 2, 3];
    let ptr: *mut u32 = s.as_mut_ptr();
    let first_value = unsafe { &mut *ptr };
    *first_value = 4;
    println!("{:?}", s); // It'll print: "[4, 2, 3]".
    //as_ref返回&
    let ptr: *const u8 = &10u8 as *const u8;
    unsafe {
        if let Some(val_back) = ptr.as_ref() {
            println!("We got back the value: {}!", val_back);
        }
    }
    let ptr: *mut u8 = &mut 10u8 as *mut u8;
    unsafe {
        if let Some(val_back) = ptr.as_ref() {
            println!("We got back the value: {}!", val_back);
        }
    }
    //is_null判断指针是否为空
    let s: &str = "Follow the rabbit";
    let ptr: *const u8 = s.as_ptr();
    assert!(!ptr.is_null());
    //sub向前移动指针
    let s: &str = "123";
    unsafe {
        let end: *const u8 = s.as_ptr().add(3);
        println!("{}", *end.sub(1) as char);
        println!("{}", *end.sub(2) as char);
    }
    //wrapping_add和add比起来更安全
// Iterate using a raw pointer in increments of two elements
    let data = [1u8, 2, 3, 4, 5];
    let mut ptr: *const u8 = data.as_ptr();
    let step = 2;
    let end_rounded_up = ptr.wrapping_add(6);
// This loop prints "1, 3, 5, "
    while ptr != end_rounded_up {
        unsafe {
            print!("{}, ", *ptr);
        }
        ptr = ptr.wrapping_add(step);
    }
    //wrapping_offset安全版的offset
// Iterate using a raw pointer in increments of two elements
    let data = [1u8, 2, 3, 4, 5];
    let mut ptr: *const u8 = data.as_ptr();
    let step = 2;
    let end_rounded_up = ptr.wrapping_offset(6);
// This loop prints "1, 3, 5, "
    while ptr != end_rounded_up {
        unsafe {
            print!("{}, ", *ptr);
        }
        ptr = ptr.wrapping_offset(step);
    }
    //wrapping_sub安全版的sub
// Iterate using a raw pointer in increments of two elements (backwards)
    let data = [1u8, 2, 3, 4, 5];
    let mut ptr: *const u8 = data.as_ptr();
    let start_rounded_down = ptr.wrapping_sub(2);
    ptr = ptr.wrapping_add(4);
    let step = 2;
// This loop prints "5, 3, 1, "
    while ptr != start_rounded_down {
        unsafe {
            print!("{}, ", *ptr);
        }
        ptr = ptr.wrapping_sub(step);
    }
}

struct Gc<T>{
    ptr_root:Cell<NonNull<GcBox<T>>>,
    marker:PhantomData<Rc<T>>,
}

struct GcBoxHeader<T>{
    roots:Cell<usize>,
    next:Option<NonNull<GcBox<T>>>,
    marked:Cell<bool>,
}

struct GcBox<T>{
    header:GcBoxHeader<T>,
    data:T,
}

//这是rust-gc里面一个奇怪的指针值操作，测试一下
//测试结果，其将指针值的尾数加1，这样做有什么意义呢？？
pub fn p3(){
    unsafe{
        let ptr=unsafe{NonNull::new_unchecked(Box::into_raw(Box::new(GcBox{
            header:GcBoxHeader{
                roots:Cell::new(1),
                marked:Cell::new(false),
                next:None,
            },
            data:100
        })))};
        let gc=Gc{
            ptr_root:Cell::new(NonNull::new_unchecked(ptr.as_ptr())),
            marker:PhantomData,
        };
        let mut ptr1=gc.ptr_root.get().as_ptr();
        println!("{:p}",ptr1);//0xe8f120
        *(&mut ptr1 as *mut *mut GcBox<i32> as *mut usize)|=1;
        println!("{:p}",ptr1);//0xe8f120
    }

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