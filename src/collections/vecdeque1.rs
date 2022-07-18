use std::collections::VecDeque;


//VecDeque
#[allow(dead_code)]
pub fn v1(){
    //new和capacity
    let _vector: VecDeque<u32> = VecDeque::new();
    let _vector: VecDeque<u32> = VecDeque::with_capacity(10);
    //append会修改两者
    let mut buf: VecDeque<_> = vec![1, 2].into_iter().collect();
    let mut buf2: VecDeque<_> = vec![3, 4].into_iter().collect();
    buf.append(&mut buf2);
    assert_eq!(buf, [1, 2, 3, 4]);
    assert!(buf2.is_empty());
    //as_mut_slices返回(&mut [T], &mut [T])，而且可以修改，如果长度为5那么就是2，3
    let mut vector = VecDeque::new();
    vector.push_back(0);
    vector.push_back(1);
    vector.push_back(2);
    vector.push_front(10);
    vector.push_front(9);
    vector.as_mut_slices().0[0] = 42;
    vector.as_mut_slices().1[0] = 24;
    assert_eq!(vector.as_slices(), (&[42, 10][..], &[24, 1,2][..]));
    //as_slices与上面类似，返回(&[T], &[T])
    let mut vector = VecDeque::new();
    vector.push_back(0);
    vector.push_back(1);
    vector.push_back(2);
    assert_eq!(vector.as_slices(), (&[0, 1, 2][..], &[][..]));
    vector.push_front(10);
    vector.push_front(9);
    assert_eq!(vector.as_slices(), (&[9, 10][..], &[0, 1, 2][..]));
    //back返回尾元素的引用，为空则返回None
    let mut d = VecDeque::new();
    assert_eq!(d.back(), None);
    d.push_back(1);
    d.push_back(2);
    assert_eq!(d.back(), Some(&2));
    //front
    let mut d = VecDeque::new();
    assert_eq!(d.front(), None);
    d.push_back(1);
    d.push_back(2);
    assert_eq!(d.front(), Some(&1));
    //back_mut上面的可变版
    let mut d = VecDeque::new();
    assert_eq!(d.back(), None);
    d.push_back(1);
    d.push_back(2);
    match d.back_mut() {
        Some(x) => *x = 9,
        None => (),
    }
    assert_eq!(d.back(), Some(&9));
    //front_mut
    let mut d = VecDeque::new();
    assert_eq!(d.front_mut(), None);
    d.push_back(1);
    d.push_back(2);
    match d.front_mut() {
        Some(x) => *x = 9,
        None => (),
    }
    assert_eq!(d.front(), Some(&9));
    //get
    let mut buf = VecDeque::new();
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    assert_eq!(buf.get(1), Some(&4));
    //get_mut
    let mut buf = VecDeque::new();
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    if let Some(elem) = buf.get_mut(1) {
        *elem = 7;
    }
    assert_eq!(buf[1], 7);
    //insert
    let mut vec_deque = VecDeque::new();
    vec_deque.push_back('a');
    vec_deque.push_back('b');
    vec_deque.push_back('c');
    assert_eq!(vec_deque, &['a', 'b', 'c']);
    vec_deque.insert(1, 'd');
    assert_eq!(vec_deque, &['a', 'd', 'b', 'c']);
    //capacity
    let buf: VecDeque<i32> = VecDeque::with_capacity(10);
    assert!(buf.capacity() >= 10);
    //len
    let mut v = VecDeque::new();
    assert_eq!(v.len(), 0);
    v.push_back(1);
    assert_eq!(v.len(), 1);
    //clear
    let mut v = VecDeque::new();
    v.push_back(1);
    v.clear();
    assert!(v.is_empty());
    //contains
    let mut vector: VecDeque<u32> = VecDeque::new();
    vector.push_back(0);
    vector.push_back(1);
    assert_eq!(vector.contains(&1), true);
    assert_eq!(vector.contains(&10), false);
    //drain排除范围值，append一样也会修改两者
    let mut v: VecDeque<_> = vec![1, 2, 3].into_iter().collect();
    let drained = v.drain(2..).collect::<VecDeque<_>>();
    assert_eq!(drained, [3]);
    assert_eq!(v, [1, 2]);
// A full range clears all contents
    v.drain(..);
    assert!(v.is_empty());
    //iter返回一个从头到尾的迭代器
    let mut buf = VecDeque::new();
    buf.push_back(5);
    buf.push_back(3);
    buf.push_back(4);
    let b: &[_] = &[&5, &3, &4];
    let c: Vec<&i32> = buf.iter().collect();
    assert_eq!(&c[..], b);
    //iter_mut
    let mut buf = VecDeque::new();
    buf.push_back(5);
    buf.push_back(3);
    buf.push_back(4);
    for num in buf.iter_mut() {
        *num = *num - 2;
    }
    let b: &[_] = &[&mut 3, &mut 1, &mut 2];
    assert_eq!(&buf.iter_mut().collect::<Vec<&mut i32>>()[..], b);
    //pop_back移除尾元素并返回
    let mut buf = VecDeque::new();
    assert_eq!(buf.pop_back(), None);
    buf.push_back(1);
    buf.push_back(3);
    assert_eq!(buf.pop_back(), Some(3));
    //pop_front移除首元素并返回
    let mut d = VecDeque::new();
    d.push_back(1);
    d.push_back(2);
    assert_eq!(d.pop_front(), Some(1));
    assert_eq!(d.pop_front(), Some(2));
    assert_eq!(d.pop_front(), None);
    //push_back尾部存入元素
    let mut buf = VecDeque::new();
    buf.push_back(1);
    buf.push_back(3);
    assert_eq!(3, *buf.back().unwrap());
    //push_front首部存入元素
    let mut d = VecDeque::new();
    d.push_front(1);
    d.push_front(2);
    assert_eq!(d.front(), Some(&2));
    //remove移除指定下标元素并返回
    let mut buf = VecDeque::new();
    buf.push_back(1);
    buf.push_back(2);
    buf.push_back(3);
    assert_eq!(buf, [1, 2, 3]);
    assert_eq!(buf.remove(1), Some(2));
    assert_eq!(buf, [1, 3]);
    //reserve
    let mut buf: VecDeque<i32> = vec![1].into_iter().collect();
    buf.reserve(10);
    assert!(buf.capacity() >= 11);
    //reserve_exact
    let mut buf: VecDeque<i32> = vec![1].into_iter().collect();
    buf.reserve_exact(10);
    assert!(buf.capacity() >= 11);
    //resize重设len
    let mut buf = VecDeque::new();
    buf.push_back(5);
    buf.push_back(10);
    buf.push_back(15);
    assert_eq!(buf, [5, 10, 15]);
    buf.resize(2, 0);
    assert_eq!(buf, [5, 10]);
    buf.resize(5, 20);
    assert_eq!(buf, [5, 10, 20, 20, 20]);
    //resize_with
    let mut buf = VecDeque::new();
    buf.push_back(5);
    buf.push_back(10);
    buf.push_back(15);
    assert_eq!(buf, [5, 10, 15]);
    buf.resize_with(5, Default::default);
    assert_eq!(buf, [5, 10, 15, 0, 0]);
    buf.resize_with(2, || unreachable!());
    assert_eq!(buf, [5, 10]);
    let mut state = 100;
    buf.resize_with(5, || { state += 1; state });
    assert_eq!(buf, [5, 10, 101, 102, 103]);
    //retain保留符合函数条件的元素
    let mut buf = VecDeque::new();
    buf.extend(1..5);
    buf.retain(|&x| x % 2 == 0);
    assert_eq!(buf, [2, 4]);
    let mut buf = VecDeque::new();
    buf.extend(1..6);
    let keep = [false, true, true, false, true];
    let mut i = 0;
    buf.retain(|_| (keep[i], i += 1).0);
    assert_eq!(buf, [2, 3, 5]);
    //rotate_left
    let mut buf: VecDeque<_> = (0..10).collect();
    buf.rotate_left(3);
    assert_eq!(buf, [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    for i in 1..10 {
        assert_eq!(i * 3 % 10, buf[0]);
        buf.rotate_left(3);
    }
    assert_eq!(buf, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    //rotate_right
    let mut buf: VecDeque<_> = (0..10).collect();
    buf.rotate_right(3);
    assert_eq!(buf, [7, 8, 9, 0, 1, 2, 3, 4, 5, 6]);
    for i in 1..10 {
        assert_eq!(0, buf[i * 3 % 10]);
        buf.rotate_right(3);
    }
    assert_eq!(buf, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    //shrink_to_fit尽可能降低capacity，比如和len一样
    let mut buf = VecDeque::with_capacity(15);
    buf.extend(0..4);
    assert_eq!(buf.capacity(), 15);
    buf.shrink_to_fit();
    assert!(buf.capacity() >= 4);
    //split_off在指定下标一分为2
    let mut buf: VecDeque<_> = vec![1,2,3].into_iter().collect();
    let buf2 = buf.split_off(1);
    assert_eq!(buf, [1]);//VecDeque是可以和数组比较的，看PartialEq<[B; N]> for VecDeque<A>
    assert_eq!(buf2, [2, 3]);
    //swap对换两个下标的元素
    let mut buf = VecDeque::new();
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    assert_eq!(buf, [3, 4, 5]);
    buf.swap(0, 2);
    assert_eq!(buf, [5, 4, 3]);
    //swap_remove_back移除指定下标值并返回，然后与最后一个值替换
    let mut buf = VecDeque::new();
    assert_eq!(buf.swap_remove_back(0), None);
    buf.push_back(1);
    buf.push_back(2);
    buf.push_back(3);
    buf.push_back(4);
    assert_eq!(buf, [1, 2, 3, 4]);
    assert_eq!(buf.swap_remove_back(0), Some(1));
    assert_eq!(buf, [4, 2, 3]);
    //swap_remove_front
    let mut buf = VecDeque::new();
    assert_eq!(buf.swap_remove_front(0), None);
    buf.push_back(1);
    buf.push_back(2);
    buf.push_back(3);
    buf.push_back(4);
    assert_eq!(buf, [1, 2, 3, 4]);
    assert_eq!(buf.swap_remove_front(2), Some(3));
    assert_eq!(buf, [2, 1, 4]);
    //truncate砍掉指定新长度外的值
    let mut buf = VecDeque::new();
    buf.push_back(5);
    buf.push_back(10);
    buf.push_back(15);
    assert_eq!(buf, [5, 10, 15]);
    buf.truncate(1);
    assert_eq!(buf, [5]);
}