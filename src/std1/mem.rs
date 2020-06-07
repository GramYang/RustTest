use std::mem;
use std::fs::File;

//mem包函数测试
pub fn t1(){
    //align_of返回类型的ABI需要的最小对齐值
    assert_eq!(4, mem::align_of::<i32>());
    println!("{}",mem::align_of::<ExampleE>());//8，不管是结构体还是枚举，都取里面最大对齐值域的最大对齐值
    //align_of_val接收的是类型实例引用
    assert_eq!(4, mem::align_of_val(&5i32));
    println!("{}",mem::align_of_val(&ExampleS::default()));//8
    //discriminant用于判断是否是同一个枚举的变体
    enum Foo { A(&'static str), B(i32), C(i32) }
    assert_eq!(mem::discriminant(&Foo::A("bar")), mem::discriminant(&Foo::A("baz")));
    assert_eq!(mem::discriminant(&Foo::B(1)), mem::discriminant(&Foo::B(2)));
    assert_ne!(mem::discriminant(&Foo::B(3)), mem::discriminant(&Foo::C(3)));
    //drop
    let v = vec![1, 2, 3];
    drop(v); // explicitly drop the vector
    use std::cell::RefCell;
    let x = RefCell::new(1);
    let mut mutable_borrow = x.borrow_mut();
    *mutable_borrow = 1;
    drop(mutable_borrow); //防止同时存在不可变借用和可变借用
    let borrow = x.borrow();
    println!("{}", *borrow);//1
    #[derive(Copy, Clone)]
    struct Foo1(u8);
    let x = 1;
    let y = Foo1(2);
    drop(x); // a copy of `x` is moved and dropped
    drop(y); // a copy of `y` is moved and dropped
    println!("x: {}, y: {}", x, y.0); // still available
    //forget手动制造内存泄漏
    let file = File::open("asset/foo.txt").unwrap();
    mem::forget(file);
    //replace将参数2移动到参数1，返回参数1的旧值
    let mut v: Vec<i32> = vec![1, 2];
    let old_v = mem::replace(&mut v, vec![3, 4, 5]);
    assert_eq!(vec![1, 2], old_v);
    assert_eq!(vec![3, 4, 5], v);
    //size_of返回类型字节数
// Some primitives
    assert_eq!(4, mem::size_of::<i32>());
    assert_eq!(8, mem::size_of::<f64>());
    assert_eq!(0, mem::size_of::<()>());
// Some arrays
    assert_eq!(8, mem::size_of::<[i32; 2]>());
    assert_eq!(12, mem::size_of::<[i32; 3]>());
    assert_eq!(0, mem::size_of::<[i32; 0]>());
// Pointer size equality
    assert_eq!(mem::size_of::<&i32>(), mem::size_of::<*const i32>());
    assert_eq!(mem::size_of::<&i32>(), mem::size_of::<Box<i32>>());
    assert_eq!(mem::size_of::<&i32>(), mem::size_of::<Option<&i32>>());
    assert_eq!(mem::size_of::<Box<i32>>(), mem::size_of::<Option<Box<i32>>>());
    //函数和函数类型的size
    fn op1(a:i32) ->i32{
        a*2
    }
    type OP = fn(a:i32) ->i32;
    println!("{}",mem::size_of::<OP>());//8
    println!("{}",mem::size_of_val(&op1));//0
    //结合size_of看到Rust如何手动优化内存布局
    #[repr(C)]
    struct FieldStruct {
        first: u8,
        second: u16,
        third: u8
    }
// The size of the first field is 1, so add 1 to the size. Size is 1.
// The alignment of the second field is 2, so add 1 to the size for padding. Size is 2.
// The size of the second field is 2, so add 2 to the size. Size is 4.
// The alignment of the third field is 1, so add 0 to the size for padding. Size is 4.
// The size of the third field is 1, so add 1 to the size. Size is 5.
// Finally, the alignment of the struct is 2 (because the largest alignment amongst its
// fields is 2), so add 1 to the size for padding. Size is 6.
    assert_eq!(6, mem::size_of::<FieldStruct>());
    #[repr(C)]
    struct TupleStruct(u8, u16, u8);
// Tuple structs follow the same rules.
    assert_eq!(6, mem::size_of::<TupleStruct>());
    // Note that reordering the fields can lower the size. We can remove both padding bytes
// by putting `third` before `second`.
    #[repr(C)]
    struct FieldStructOptimized {
        first: u8,
        third: u8,
        second: u16
    }
    assert_eq!(4, mem::size_of::<FieldStructOptimized>());
    // Union size is the size of the largest field.
    #[repr(C)]
    union ExampleUnion {
        smaller: u8,
        larger: u16
    }
    assert_eq!(2, mem::size_of::<ExampleUnion>());
    //size_of_val接收类型值引用
    assert_eq!(4, mem::size_of_val(&5i32));
    let x: [u8; 13] = [0; 13];
    let y: &[u8] = &x;
    assert_eq!(13, mem::size_of_val(y));
    //swap交换两个可变引用的值而无需解初始化
    let mut x = 5;
    let mut y = 42;
    mem::swap(&mut x, &mut y);
    assert_eq!(42, x);
    assert_eq!(5, y);
    //take将参数重设为默认值，返回参数值
    let mut v: Vec<i32> = vec![1, 2];
    let old_v = mem::take(&mut v);
    assert_eq!(vec![1, 2], old_v);
    assert!(v.is_empty());
    //transmute将一个类型转换为另一个类型，这两个类型必须size相同，类似于C的memcpy
    fn foo() -> i32 {
        0
    }
    let pointer = foo as *const ();
    let function = unsafe {
        std::mem::transmute::<*const (), fn() -> i32>(pointer)
    };
    assert_eq!(function(), 0);
    let ptr = &0;
    let ptr_num_transmute = unsafe {
        std::mem::transmute::<&i32, usize>(ptr)
    };
// Use an `as` cast instead
    let ptr_num_cast = ptr as *const i32 as usize;
    let ptr: *mut i32 = &mut 0;
    let ref_transmuted = unsafe {
        std::mem::transmute::<*mut i32, &mut i32>(ptr)
    };
// Use a reborrow instead
    let ref_casted = unsafe { &mut *ptr };
    let ptr = &mut 0;
    let val_transmuted = unsafe {
        std::mem::transmute::<&mut i32, &mut u32>(ptr)
    };
// Now, put together `as` and reborrowing - note the chaining of `as`
// `as` is not transitive
    let val_casts = unsafe { &mut *(ptr as *mut i32 as *mut u32) };
// this is not a good way to do this.
    let slice = unsafe { std::mem::transmute::<&str, &[u8]>("Rust") };
    assert_eq!(slice, &[82, 117, 115, 116]);
// You could use `str::as_bytes`
    let slice = "Rust".as_bytes();
    assert_eq!(slice, &[82, 117, 115, 116]);
// Or, just use a byte string, if you have control over the string
// literal
    assert_eq!(b"Rust", &[82, 117, 115, 116]);
    //transmute_copy，不会move值
    #[repr(packed)]
    struct Foo2 {
        bar: u8,
    }
    let foo_array = [10u8];
    unsafe {
        // Copy the data from 'foo_array' and treat it as a 'Foo'
        let mut foo_struct: Foo2 = mem::transmute_copy(&foo_array);
        assert_eq!(foo_struct.bar, 10);
        // Modify the copied data
        foo_struct.bar = 20;
        assert_eq!(foo_struct.bar, 20);
    }
// The contents of 'foo_array' should not have changed
    assert_eq!(foo_array, [10]);
    //zeroed返回该类型的所有字节都是0的值
    let x: i32 = unsafe { mem::zeroed() };
    assert_eq!(0, x);
}

#[derive(Default)]
struct ExampleS{
    a:i32,
    b:String,
    c:bool,
}

enum ExampleE{
    A(String,String,i32),
    B(i32),
}
