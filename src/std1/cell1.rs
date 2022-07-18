use std::{
    cell::{Cell, RefCell, UnsafeCell},
    rc::Rc,
};

//RefCell
#[allow(dead_code)]
pub fn c1() {
    let c = RefCell::new(5);
    //可以有多个borrow
    let borrow_c1 = c.borrow();
    let borrow_c2 = c.borrow();
    println!("{} {}", borrow_c1, borrow_c2);
    //只能有一个borrow_mut，borrow和borrow_mut不能共存
    let c = RefCell::new(5);
    *c.borrow_mut() = 7;
    assert_eq!(*c.borrow(), 7);
    //get_mut和borrow_mut类似，区别是c必须是mut的，官方建议使用borrow_mut
    let mut c = RefCell::new(5);
    *c.get_mut() += 1;
    assert_eq!(c, RefCell::new(6));
    //into_inner消费RefCell，返回其中包裹的值
    let c = RefCell::new(5);
    let five = c.into_inner();
    println!("{}", five);
    //replace替换内部的值并返回旧值
    let cell = RefCell::new(5);
    let old_value = cell.replace(6);
    assert_eq!(old_value, 5);
    assert_eq!(cell, RefCell::new(6));
    //replace_with
    let cell = RefCell::new(5);
    let old_value = cell.replace_with(|&mut old| old + 1);
    assert_eq!(old_value, 5);
    assert_eq!(cell, RefCell::new(6));
    //swap替换两个RefCell的包裹值
    let c = RefCell::new(5);
    let d = RefCell::new(6);
    c.swap(&d);
    assert_eq!(c, RefCell::new(6));
    assert_eq!(d, RefCell::new(5));
    //try_borrow不可变借用内部值，如果事先borrow_mut了则返回Err
    let c = RefCell::new(5);
    {
        let _m = c.borrow_mut();
        assert!(c.try_borrow().is_err());
    }
    {
        let _m = c.borrow();
        assert!(c.try_borrow().is_ok());
    }
    //try_borrow_mut可变借用内部值，不能和borrow共存，只能borrow_mut一次
    let c = RefCell::new(5);
    {
        let _m = c.borrow();
        assert!(c.try_borrow_mut().is_err());
    }
    assert!(c.try_borrow_mut().is_ok());
}

//Cell
#[allow(dead_code)]
pub fn c2() {
    //get和set
    struct SomeStruct {
        regular_field: u8,
        special_field: Cell<u8>,
    }
    let new_value = 100;
    let my_struct = SomeStruct {
        regular_field: 0,
        special_field: Cell::new(1),
    };
    my_struct.special_field.set(new_value);
    assert_eq!(my_struct.special_field.get(), new_value);
    //as_ptr
    let c = Cell::new(2);
    let ptr = c.as_ptr();
    println!("{:p}", ptr);
    //as_slice_of_cells根据&Cell<[T]>获取&[Cell<T>]
    let slice: &mut [i32] = &mut [1, 2, 3];
    let cell_slice: &Cell<[i32]> = Cell::from_mut(slice);
    let slice_cell: &[Cell<i32>] = cell_slice.as_slice_of_cells();
    assert_eq!(slice_cell.len(), 3);
    //from_mut根据&mut T获取&Cell<T>
    let slice: &mut [i32] = &mut [1, 2, 3];
    let cell_slice: &Cell<[i32]> = Cell::from_mut(slice);
    let slice_cell: &[Cell<i32>] = cell_slice.as_slice_of_cells();
    assert_eq!(slice_cell.len(), 3);
    //get
    let c = Cell::new(5);
    let five = c.get();
    println!("{}", five);
    //get_mut返回的是里面值的可变引用
    let mut c = Cell::new(5);
    *c.get_mut() += 1;
    assert_eq!(c.get(), 6);
    //into_inner类似unwrap
    let c = Cell::new(5);
    let five = c.into_inner();
    assert_eq!(five, 5);
    //replace替换包裹值并返回旧值
    let cell = Cell::new(5);
    assert_eq!(cell.get(), 5);
    assert_eq!(cell.replace(10), 5);
    assert_eq!(cell.get(), 10);
    //set设置包裹的值
    let c = Cell::new(5);
    c.set(10);
    //swap
    let c1 = Cell::new(5i32);
    let c2 = Cell::new(10i32);
    c1.swap(&c2);
    assert_eq!(10, c1.get());
    assert_eq!(5, c2.get());
    //take
    let c = Cell::new(5);
    let five = c.take();
    assert_eq!(five, 5);
    assert_eq!(c.into_inner(), 0);
}

//UnsafeCell
//一般来说，将一个&T类型转换成一个&mut T被认为是未定义的行为，UnsafeCell是做到这个唯一的合法方法。
// 如Cell<T>和RefCell<T>，都使用UnsafeCell来包装它们的内部数据。
// 如果你有一个引用&SomeStruct，在Rust中，SomeStruct的所有字段都是不可变的。即使使用UnsafeCell<T>，也没有包含&mut的合法方法。
//UnsafeCell的原理很简单：就是用裸指针*mut T来指向内容，需要你自己来正确的使用他。
//如果你将UnsafeCell里的*mut T转换为&T，那么T必须保持不变直到引用的生命周期结束。
#[allow(dead_code)]
pub fn c3() {
    //get返回一个内部值的可变裸指针*mut T
    let uc = UnsafeCell::new(5);
    let _five = uc.get();
    //into_inner类似unwrap
    let uc = UnsafeCell::new(5);
    let _five = uc.into_inner();
}

//RefCell实现链表
#[allow(dead_code)]
pub fn c4() {
    #[derive(Debug)]
    enum List1 {
        Cons(Rc<RefCell<i32>>, Rc<List1>),
        Nil,
    }
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(List1::Cons(Rc::clone(&value), Rc::new(List1::Nil)));
    let b = List1::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = List1::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("{:?}", a); //Cons(RefCell { value: 15 }, Nil)
    println!("{:?}", b); //Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
    println!("{:?}", c); //Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
}
