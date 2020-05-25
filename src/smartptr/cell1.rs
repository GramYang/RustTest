use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut, Cell, UnsafeCell};

//RefCell使用例子
pub fn c_t1(){
    let value =Rc::new(RefCell::new(5));
    let a=Rc::new(List1::Cons(Rc::clone(&value),Rc::new(List1::Nil)));
    let b=List1::Cons(Rc::new(RefCell::new(6)),Rc::clone(&a));
    let c=List1::Cons(Rc::new(RefCell::new(10)),Rc::clone(&a));
    *value.borrow_mut() +=10;
    println!("{:?}",a); //Cons(RefCell { value: 15 }, Nil)
    println!("{:?}",b); //Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
    println!("{:?}",c); //Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
}

#[derive(Debug)]
enum List1{
    Cons(Rc<RefCell<i32>>,Rc<List1>),
    Nil,
}

//RefCell基本使用
pub fn c_t2(){
    //borrow, 可以有多个borrow
    let c = RefCell::new(5);
    let borrowed_five = c.borrow();
    let borrowed_five2 = c.borrow();
    //borrow_mut，只能有一个borrow_mut
    let c = RefCell::new(5);
    *c.borrow_mut() = 7;
    assert_eq!(*c.borrow(), 7);
    //borrow和borrow_mut不能共存
    // let result = thread::spawn(move || {
    //     let c = RefCell::new(5);
    //     let m = c.borrow_mut();
    //     let b = c.borrow(); // this causes a panic
    // }).join();
    // assert!(result.is_err());
    //get_mut和borrow_mut类似，区别是c必须是mut的，官方建议使用borrow_mut
    let mut c = RefCell::new(5);
    *c.get_mut() += 1;
    assert_eq!(c, RefCell::new(6));
    //into_inner消费RefCell，返回其中包裹的值
    let c = RefCell::new(5);
    let five = c.into_inner();
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
        let m = c.borrow_mut();
        assert!(c.try_borrow().is_err());
    }
    {
        let m = c.borrow();
        assert!(c.try_borrow().is_ok());
    }
    //try_borrow_mut可变借用内部值，不能和borrow共存，只能borrow_mut一次
    let c = RefCell::new(5);
    {
        let m = c.borrow();
        assert!(c.try_borrow_mut().is_err());
    }
    assert!(c.try_borrow_mut().is_ok());
}

//RefCell的borrow返回的Ref使用测试
//Ref是不可变引用，所以可以clone
pub fn c_t3(){
    //map返回一个新的Ref
    let c = RefCell::new((5, 'b'));
    let b1: Ref<(u32, char)> = c.borrow();
    let x1 = *b1;
    let b2: Ref<u32> = Ref::map(b1, |t| &t.0);
    assert_eq!(*b2, 5);
    //map_split拆分Ref成多个Ref
    let cell = RefCell::new([1, 2, 3, 4]);
    let borrow = cell.borrow();
    let (begin, end) = Ref::map_split(borrow, |slice| slice.split_at(2));
    assert_eq!(*begin, [1, 2]);
    assert_eq!(*end, [3, 4]);
}

//RefCell的borrow_mut返回的RefMut使用测试
//Ref是可变引用，不可以clone
pub fn c_t4(){
    //map
    let c = RefCell::new((5, 'b'));
    {
        let b1: RefMut<(u32, char)> = c.borrow_mut();
        let mut b2: RefMut<u32> = RefMut::map(b1, |t| &mut t.0);
        assert_eq!(*b2, 5);
        *b2 = 42;
    }
    assert_eq!(*c.borrow(), (42, 'b'));
    //map_split
    let cell = RefCell::new([1, 2, 3, 4]);
    let borrow = cell.borrow_mut();
    let (mut begin, mut end) = RefMut::map_split(borrow, |slice| slice.split_at_mut(2));
    assert_eq!(*begin, [1, 2]);
    assert_eq!(*end, [3, 4]);
    begin.copy_from_slice(&[4, 3]);
    end.copy_from_slice(&[2, 1]);
    for i in (*begin).iter(){
        println!("{}",i);
    }
}

//RefCell内部持有的Cell, Cell使得不变的结构体内的域可变
pub fn c_t5(){
    //基本使用
    struct SomeStruct {
        regular_field: u8,
        special_field: Cell<u8>,
    }
    let my_struct = SomeStruct {
        regular_field: 0,
        special_field: Cell::new(1),
    };
    let new_value = 100;
    my_struct.special_field.set(new_value);
    assert_eq!(my_struct.special_field.get(), new_value);
    //as_ptr返回*mut T
    let c = Cell::new(5);
    let ptr = c.as_ptr();
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
    //get返回的是里面包裹值的copy
    let c = Cell::new(5);
    let five = c.get();
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
pub fn c_t6(){
    //get返回一个内部值的可变裸指针*mut T
    let uc = UnsafeCell::new(5);
    let five = uc.get();
    //into_inner类似unwrap
    let uc = UnsafeCell::new(5);
    let five = uc.into_inner();
}