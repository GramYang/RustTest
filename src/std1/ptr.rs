use std::{ptr, mem};
use std::rc::Rc;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

//std::ptr包函数测试
pub fn p1(){
    //copy拷贝参数3 * size_of::<T>()个字节从参数1到参数2
    #[allow(dead_code)]
    unsafe fn from_buf_raw<T>(ptr: *const T, elts: usize) -> Vec<T> {//从ptr指向的数组copy elts个字节到一个新数组并返回数组
        let mut dst = Vec::with_capacity(elts);
        dst.set_len(elts);
        ptr::copy(ptr, dst.as_mut_ptr(), elts);
        dst
    }
    //copy_nonoverlapping上面的无重叠版
    //将src所有元素移动到dst，src置空
    fn append<T>(dst: &mut Vec<T>, src: &mut Vec<T>) {
        let src_len = src.len();
        let dst_len = dst.len();
        //dst预留足够空间
        dst.reserve(src_len);
        unsafe {
            //移动dst指针，使得不重叠
            let dst_ptr = dst.as_mut_ptr().offset(dst_len as isize);
            let src_ptr = src.as_ptr();
            src.set_len(0);
            ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_len);
            dst.set_len(dst_len + src_len);
        }
    }
    let mut a = vec!['r'];
    let mut b = vec!['u', 's', 't'];
    append(&mut a, &mut b);
    assert_eq!(a, &['r', 'u', 's', 't']);
    assert!(b.is_empty());
    //drop_in_place执行指针指向的值的析构器
    let last = Rc::new(1);
    let weak = Rc::downgrade(&last);
    let mut v = vec![Rc::new(0), last];
    unsafe {
        //获取v[1]的裸指针
        let ptr = &mut v[1] as *mut _;
        //把v[1]排除在外，避免被drop
        v.set_len(1);
        //使用drop_in_place才能避免内存泄漏，不然v[1]永远不会被drop
        ptr::drop_in_place(ptr);
    }
    assert_eq!(v, &[0.into()]);
    assert!(weak.upgrade().is_none());//确认v[1]已经被drop
    //eq比较裸指针是否相等
    let five = 5;
    let other_five = 5;
    let five_ref = &five;
    let same_five_ref = &five;
    let other_five_ref = &other_five;
    assert!(five_ref == same_five_ref);
    assert!(ptr::eq(five_ref, same_five_ref));
    assert!(five_ref == other_five_ref);
    assert!(!ptr::eq(five_ref, other_five_ref));
    //肥指针也可以进行比较
    let a = [1, 2, 3];
    assert!(std::ptr::eq(&a[..3], &a[..3]));
    assert!(!std::ptr::eq(&a[..2], &a[..3]));
    assert!(!std::ptr::eq(&a[0..2], &a[1..3]));
    //比较trait，不同的*const dyn Trait不同，但是*const u8都是相同的
    #[repr(transparent)]
    struct Wrapper { member: i32 }
    trait Trait {}
    impl Trait for Wrapper {}
    impl Trait for i32 {}
    let wrapper = Wrapper { member: 10 };
// Pointers have equal addresses.
    assert!(std::ptr::eq(
        &wrapper as *const Wrapper as *const u8,
        &wrapper.member as *const i32 as *const u8
    ));
// Objects have equal addresses, but `Trait` has different implementations.
    assert!(!std::ptr::eq(
        &wrapper as &dyn Trait,
        &wrapper.member as &dyn Trait,
    ));
    assert!(!std::ptr::eq(
        &wrapper as &dyn Trait as *const dyn Trait,
        &wrapper.member as &dyn Trait as *const dyn Trait,
    ));
// Converting the reference to a `*const u8` compares by address.
    assert!(std::ptr::eq(
        &wrapper as &dyn Trait as *const dyn Trait as *const u8,
        &wrapper.member as &dyn Trait as *const dyn Trait as *const u8,
    ));
    //hash
    let five = 5;
    let five_ref = &five;
    let mut hasher = DefaultHasher::new();
    ptr::hash(five_ref, &mut hasher);
    let actual = hasher.finish();
    let mut hasher = DefaultHasher::new();
    (five_ref as *const i32).hash(&mut hasher);
    let expected = hasher.finish();
    assert_eq!(actual, expected);
    //null创建一个空裸指针
    let p: *const i32 = ptr::null();
    assert!(p.is_null());
    let p: *mut i32 = ptr::null_mut();
    assert!(p.is_null());
    //read读取裸指针指向的值但是不消耗此指针
    let x = 12;
    let y = &x as *const i32;
    unsafe {
        assert_eq!(std::ptr::read(y), 12);
    }
    let mut s = String::from("foo");
    unsafe {
        //s2和s都指向同一个内存
        let mut s2: String = ptr::read(&s);
        assert_eq!(s2, "foo");
        //s2重新赋值，那么"foo"就被drop了，因为s也不能用了
        s2 = String::default();
        assert_eq!(s2, "");
        //不能给s赋值，因为s的底层内存已经drop了，你再给s赋值就会重复free内存导致错误
        // s = String::from("bar"); // ERROR
        //可以使用ptr::write向s重新写入一个值
        ptr::write(&mut s, String::from("bar"));
    }
    assert_eq!(s, "bar");
    //replace替换指针指向的值并返回该指针的旧值
    let mut rust = vec!['b', 'u', 's', 't'];
// `mem::replace` would have the same effect without requiring the unsafe
// block.
    let b = unsafe {
        ptr::replace(&mut rust[0], 'r')
    };
    assert_eq!(b, 'b');
    assert_eq!(rust, &['r', 'u', 's', 't']);
    //swap交换两个可变指针的值，两个值必须是同一类型，可以重叠
    let mut array = [0, 1, 2, 3];
    let x = array[0..].as_mut_ptr() as *mut [u32; 2]; // this is `array[0..2]`
    let y = array[2..].as_mut_ptr() as *mut [u32; 2]; // this is `array[2..4]`
    unsafe {
        ptr::swap(x, y);
        assert_eq!([2, 3, 0, 1], array);
    }
    let mut array = [0, 1, 2, 3];
    let x = array[0..].as_mut_ptr() as *mut [u32; 3]; // this is `array[0..3]`
    let y = array[1..].as_mut_ptr() as *mut [u32; 3]; // this is `array[1..4]`
    unsafe {
        ptr::swap(x, y);
        // The indices `1..3` of the slice overlap between `x` and `y`.
        // Reasonable results would be for to them be `[2, 3]`, so that indices `0..3` are
        // `[1, 2, 3]` (matching `y` before the `swap`); or for them to be `[0, 1]`
        // so that indices `1..4` are `[0, 1, 2]` (matching `x` before the `swap`).
        // This implementation is defined to make the latter choice.
        assert_eq!([1, 0, 1, 2], array);
    }
    //swap_nonoverlapping
    let mut x = [1, 2, 3, 4];
    let mut y = [7, 8, 9];
    unsafe {
        ptr::swap_nonoverlapping(x.as_mut_ptr(), y.as_mut_ptr(), 2);
    }
    assert_eq!(x, [7, 8, 3, 4]);
    assert_eq!(y, [1, 2, 9]);
    //write在不重写和drop旧值的情况下覆写指针指向的值
    let mut x = 0;
    let y = &mut x as *mut i32;
    let z = 12;
    unsafe {
        std::ptr::write(y, z);
        assert_eq!(std::ptr::read(y), 12);
    }
    fn swap<T>(a: &mut T, b: &mut T) {
        unsafe {
            //tmp是a的值的copy
            let tmp = ptr::read(a);
            // Exiting at this point (either by explicitly returning or by
            // calling a function which panics) would cause the value in `tmp` to
            // be dropped while the same value is still referenced by `a`. This
            // could trigger undefined behavior if `T` is not `Copy`.
            // Create a bitwise copy of the value at `b` in `a`.
            // This is safe because mutable references cannot alias.
            ptr::copy_nonoverlapping(b, a, 1);
            // As above, exiting here could trigger undefined behavior because
            // the same value is referenced by `a` and `b`.
            // Move `tmp` into `b`.
            ptr::write(b, tmp);
            // `tmp` has been moved (`write` takes ownership of its second argument),
            // so nothing is dropped implicitly here.
        }
    }
    let mut foo = "foo".to_owned();
    let mut bar = "bar".to_owned();
    swap(&mut foo, &mut bar);
    assert_eq!(foo, "bar");
    assert_eq!(bar, "foo");
}