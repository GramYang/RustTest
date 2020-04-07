
//测试LocalKey
pub fn test1(){
    use std::cell::RefCell;
    use std::thread;
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });
    let t = thread::spawn(move|| {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });
    t.join().unwrap();
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
}