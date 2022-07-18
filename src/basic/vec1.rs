
//Vec
#[allow(dead_code)]
pub fn v1(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);//下标
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);
    vec[0] = 7;
    assert_eq!(vec[0], 7);
    vec.extend([1, 2, 3].iter().copied());
    for x in &vec {
        print!("{} ", x);
    }//7 1 2 3
    assert_eq!(vec, [7, 1, 2, 3]);
    //vec!宏使用
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    assert_eq!(vec, [1, 2, 3, 4]);
    //resize初始化
    let mut vec1 = Vec::with_capacity(5);
    vec1.resize(5, 0);
    println!("{} {}",vec1.len(),vec1.capacity());//5 5
    //push和pop模拟栈
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        // Prints 3, 2, 1
        print!("{} ", top);
    }
    //index寻址，index越界会panic
    let v = vec![0, 2, 4, 6];
    println!("{}", v[1]); //2
    //长度容量
    let a = vec![1, 2, 3];
    println!("{} {}",a.len(),a.capacity());//3 3
    //clear清空vec
    let mut v = vec![1, 2, 3];
    v.clear();
    assert!(v.is_empty());
    let v = Vec::<i32>::new();
    assert!(v.is_empty());
    //append修改两者
    let mut vec = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);//append修改两者
    println!("{:?} {:?}",vec,vec2);//[1, 2, 3, 4, 5, 6] []
    //drain也是修改两者，范围参数，这种修改两者的方法都是靠的移动指针来完成的
    let mut v = vec![1, 2, 3];
    let u: Vec<_> = v.drain(1..).collect();
    assert_eq!(v, &[1]);
    assert_eq!(u, &[2, 3]);
    v.drain(..);
    println!("{:?}",v);//[]
    //dedup去除重复元素
    let mut vec = vec![1, 2, 2, 3, 2];
    vec.dedup();
    assert_eq!(vec, [1, 2, 3, 2]);
    //extend_from_slice将切片的元素clone到vec中
    let mut vec = vec![1];
    let vec1 = vec![2,3,4];
    vec.extend_from_slice(&vec1);
    println!("{:?} {:?}",vec,vec1);//[1, 2, 3, 4] [2, 3, 4]
    drop(vec1);
    println!("{:?}",vec);//[1, 2, 3, 4]，drop了vec1后值不变，说明有clone
    //insert在指定下标插入元素并右移右边的元素
    let mut vec = vec![1, 2, 3];
    vec.insert(1, 4);
    assert_eq!(vec, [1, 4, 2, 3]);
    vec.insert(4, 5);
    assert_eq!(vec, [1, 4, 2, 3, 5]);
    //remove移除并返回指定下标元素，右边元素全部左移
    let mut v = vec![1, 2, 3];
    assert_eq!(v.remove(1), 2);
    assert_eq!(v, [1, 3]);
    //resize重新设置vec的长度
    let mut vec = vec!["hello"];
    vec.resize(3, "world");
    assert_eq!(vec, ["hello", "world", "world"]);
    let mut vec = vec![1, 2, 3, 4];
    vec.resize(2, 0);
    assert_eq!(vec, [1, 2]);
    //retain根据条件函数删除元素
    let mut vec = vec![1, 2, 3, 4, 5];
    let keep = [false, true, true, false, true];
    let mut i = 0;
    vec.retain(|_| (keep[i], i += 1).0);
    assert_eq!(vec, [2, 3, 5]);
    //get、first取元素引用
    let v = [10, 40, 30];
    assert_eq!(Some(&10), v.first());
    let w: &[i32] = &[];
    assert_eq!(None, w.first());
    let x = &mut [0, 1, 2];
    if let Some(first) = x.first_mut() {
        *first = 5;
    }
    assert_eq!(x, &[5, 1, 2]);
    let v = [10, 40, 30];
    assert_eq!(Some(&40), v.get(1));
    assert_eq!(Some(&[10, 40][..]), v.get(0..2));
    assert_eq!(None, v.get(3));
    assert_eq!(None, v.get(0..4));
    let x = &mut [0, 1, 2];
    if let Some(elem) = x.get_mut(1) {
        *elem = 42;
    }
    assert_eq!(x, &[0, 42, 2]);
}

//Vec整合[T]，也就是切片的方法
#[allow(dead_code)]
pub fn v2(){
    //as_mut_ptr返回切片buffer的一个unsafe的*mut T
    let x = &mut [1, 2, 4];
    let x_ptr = x.as_mut_ptr();
    unsafe {
        for i in 0..x.len() {
            *x_ptr.add(i) += 2;
        }
    }
    assert_eq!(x, &[3, 4, 6]);
    //as_ptr返回切片buffer的*const T
    let x = &[1, 2, 4];
    let x_ptr = x.as_ptr();
    unsafe {
        for i in 0..x.len() {
            assert_eq!(x.get_unchecked(i), &*x_ptr.add(i));
        }
    }
    //binary_search二叉树搜索有序切片中的指定值
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    assert_eq!(s.binary_search(&13),  Ok(9));
    assert_eq!(s.binary_search(&4),   Err(7));
    assert_eq!(s.binary_search(&100), Err(13));
    let r = s.binary_search(&1);
    assert!(match r { Ok(1..=4) => true, _ => false, });
    //插入一个值到有序vec中
    let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    let num = 42;
    let idx = s.binary_search(&num).unwrap_or_else(|x| x);
    s.insert(idx, num);
    assert_eq!(s, [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
    //binary_search_by执行一个函数
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    let seek = 13;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
    let seek = 4;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(7));
    let seek = 100;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(13));
    let seek = 1;
    let r = s.binary_search_by(|probe| probe.cmp(&seek));
    assert!(match r { Ok(1..=4) => true, _ => false, });
    //binary_search_by_key选择一个key
    let s = [(0, 0), (2, 1), (4, 1), (5, 1), (3, 1),
        (1, 2), (2, 3), (4, 5), (5, 8), (3, 13),
        (1, 21), (2, 34), (4, 55)];
    assert_eq!(s.binary_search_by_key(&13, |&(_a,b)| b),  Ok(9));
    assert_eq!(s.binary_search_by_key(&4, |&(_a,b)| b),   Err(7));
    assert_eq!(s.binary_search_by_key(&100, |&(_a,b)| b), Err(13));
    let r = s.binary_search_by_key(&1, |&(_a,b)| b);
    assert!(match r { Ok(1..=4) => true, _ => false, });
    //chunks返回分块切片
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.chunks(2);
    assert_eq!(iter.next().unwrap(), &['l', 'o']);
    assert_eq!(iter.next().unwrap(), &['r', 'e']);
    assert_eq!(iter.next().unwrap(), &['m']);
    assert!(iter.next().is_none());
    //rchunks
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.rchunks(2);
    assert_eq!(iter.next().unwrap(), &['e', 'm']);
    assert_eq!(iter.next().unwrap(), &['o', 'r']);
    assert_eq!(iter.next().unwrap(), &['l']);
    assert!(iter.next().is_none());
    //chunks_mut可修改
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;
    for chunk in v.chunks_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[1, 1, 2, 2, 3]);
    //rchunks_mut
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;
    for chunk in v.rchunks_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[3, 2, 2, 1, 1]);
    //chunks_exact将小于size的分片区分开
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.chunks_exact(2);
    assert_eq!(iter.next().unwrap(), &['l', 'o']);
    assert_eq!(iter.next().unwrap(), &['r', 'e']);
    assert!(iter.next().is_none());
    assert_eq!(iter.remainder(), &['m']);
    //rchunks_exact
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.rchunks_exact(2);
    assert_eq!(iter.next().unwrap(), &['e', 'm']);
    assert_eq!(iter.next().unwrap(), &['o', 'r']);
    assert!(iter.next().is_none());
    assert_eq!(iter.remainder(), &['l']);
    //chunks_exact_mut分片可以修改
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;
    for chunk in v.chunks_exact_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[1, 1, 2, 2, 0]);
    //rchunks_exact_mut
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;
    for chunk in v.rchunks_exact_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[0, 2, 2, 1, 1]);
    //clone_from_slice将src的数组复制到dst，复制的数组必须长度一致，如果实现了Copy则可以使用copy_from_slice
    let src = [1, 2, 3, 4];
    let mut dst = [0, 0];
    dst.clone_from_slice(&src[2..]);
    assert_eq!(src, [1, 2, 3, 4]);
    assert_eq!(dst, [3, 4]);
    //copy_from_slice
    let src = [1, 2, 3, 4];
    let mut dst = [0, 0];
    dst.copy_from_slice(&src[2..]);
    assert_eq!(src, [1, 2, 3, 4]);
    assert_eq!(dst, [3, 4]);
    //copy_within数组内复制粘贴元素
    let mut bytes = *b"Hello, World!";
    bytes.copy_within(1..5, 8);
    assert_eq!(&bytes, b"Hello, Wello!");
    //concat拼接
    assert_eq!(["hello", "world"].concat(), "helloworld");
    assert_eq!([[1, 2], [3, 4]].concat(), [1, 2, 3, 4]);
    //starts_with
    let v = [10, 40, 30];
    assert!(v.starts_with(&[10]));
    assert!(v.starts_with(&[10, 40]));
    assert!(!v.starts_with(&[50]));
    assert!(!v.starts_with(&[10, 50]));
    let v = &[10, 40, 30];
    assert!(v.starts_with(&[]));
    let v: &[u8] = &[];
    assert!(v.starts_with(&[]));
    //ends_with
    let v = [10, 40, 30];
    assert!(v.ends_with(&[30]));
    assert!(v.ends_with(&[40, 30]));
    assert!(!v.ends_with(&[50]));
    assert!(!v.ends_with(&[50, 30]));
    //空切片总是true
    let v = &[10, 40, 30];
    assert!(v.ends_with(&[]));
    let v: &[u8] = &[];
    assert!(v.ends_with(&[]));
    //first返回第一个值的引用，没有则返回None
    let v = [10, 40, 30];
    assert_eq!(Some(&10), v.first());
    let w: &[i32] = &[];
    assert_eq!(None, w.first());
    //last
    let v = [10, 40, 30];
    assert_eq!(Some(&30), v.last());
    let w: &[i32] = &[];
    assert_eq!(None, w.last());
    //first_mut
    let x = &mut [0, 1, 2];
    if let Some(first) = x.first_mut() {
        *first = 5;
    }
    assert_eq!(x, &[5, 1, 2]);
    //last_mut
    let x = &mut [0, 1, 2];
    if let Some(last) = x.last_mut() {
        *last = 10;
    }
    assert_eq!(x, &[0, 1, 10]);
    //get
    let v = [10, 40, 30];
    assert_eq!(Some(&40), v.get(1));
    assert_eq!(Some(&[10, 40][..]), v.get(0..2));
    assert_eq!(None, v.get(3));
    assert_eq!(None, v.get(0..4));
    //get_mut
    let x = &mut [0, 1, 2];
    if let Some(elem) = x.get_mut(1) {
        *elem = 42;
    }
    assert_eq!(x, &[0, 42, 2]);
    //get_unchecked返回切片值引用，不做边界检查
    let x = &[1, 2, 4];
    unsafe {
        assert_eq!(x.get_unchecked(1), &2);
    }
    //get_unchecked_mut
    let x = &mut [1, 2, 4];
    unsafe {
        let elem = x.get_unchecked_mut(1);
        *elem = 13;
    }
    assert_eq!(x, &[1, 13, 4]);
    //is_empty
    let a = [1, 2, 3];
    assert!(!a.is_empty());
    //iter
    let x = &[1, 2, 4];
    let mut iterator = x.iter();
    assert_eq!(iterator.next(), Some(&1));
    assert_eq!(iterator.next(), Some(&2));
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), None);
    //iter_mut
    let x = &mut [1, 2, 4];
    for elem in x.iter_mut() {
        *elem += 2;
    }
    assert_eq!(x, &[3, 4, 6]);
    //join以指定字符为间隔拼接
    assert_eq!(["hello", "world"].join(" "), "hello world");
    assert_eq!([[1, 2], [3, 4]].join(&0), [1, 2, 0, 3, 4]);
    assert_eq!([[1, 2], [3, 4]].join(&[0, 0][..]), [1, 2, 0, 0, 3, 4]);
    //len
    let a = [1, 2, 3];
    assert_eq!(a.len(), 3);
    //repeat重复n次
    assert_eq!([1, 2].repeat(3), vec![1, 2, 1, 2, 1, 2]);
    //reverse反转
    let mut v = [1, 2, 3];
    v.reverse();
    assert!(v == [3, 2, 1]);
    //rotate_left
    let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
    a.rotate_left(2);
    assert_eq!(a, ['c', 'd', 'e', 'f', 'a', 'b']);
    let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
    a[1..5].rotate_left(1);
    assert_eq!(a, ['a', 'c', 'd', 'e', 'b', 'f']);
    //rotate_right
    let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
    a.rotate_right(2);
    assert_eq!(a, ['e', 'f', 'a', 'b', 'c', 'd']);
    let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
    a[1..5].rotate_right(1);
    assert_eq!(a, ['a', 'e', 'b', 'c', 'd', 'f']);
    //split拆分数组
    let slice = [10, 40, 33, 20];
    let mut iter = slice.split(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[10, 40]);
    assert_eq!(iter.next().unwrap(), &[20]);
    assert!(iter.next().is_none());
    let slice = [10, 40, 33];
    let mut iter = slice.split(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[10, 40]);
    // assert_eq!(iter.next().unwrap(), &[]);
    println!("{:?}",iter.next().unwrap());
    assert!(iter.next().is_none());
    let slice = [10, 6, 33, 20];
    let mut iter = slice.split(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[10]);
    // assert_eq!(iter.next().unwrap(), &[]);
    println!("{:?}",iter.next().unwrap());
    assert_eq!(iter.next().unwrap(), &[20]);
    assert!(iter.next().is_none());
    //rsplit
    let slice = [11, 22, 33, 0, 44, 55];
    let mut iter = slice.rsplit(|num| *num == 0);
    assert_eq!(iter.next().unwrap(), &[44, 55]);
    assert_eq!(iter.next().unwrap(), &[11, 22, 33]);
    assert_eq!(iter.next(), None);
    let v = &[0, 1, 1, 2, 3, 5, 8];
    let mut it = v.rsplit(|n| *n % 2 == 0);
    // assert_eq!(it.next().unwrap(), &[]);
    println!("{:?}",it.next().unwrap());
    assert_eq!(it.next().unwrap(), &[3, 5]);
    assert_eq!(it.next().unwrap(), &[1, 1]);
    // assert_eq!(it.next().unwrap(), &[]);
    println!("{:?}",it.next().unwrap());
    assert_eq!(it.next(), None);
    //split_at按下标拆分数组
    let v = [1, 2, 3, 4, 5, 6];
    {
        let (_left, right) = v.split_at(0);
        // assert!(left == []);
        assert!(right == [1, 2, 3, 4, 5, 6]);
    }
    {
        let (left, right) = v.split_at(2);
        assert!(left == [1, 2]);
        assert!(right == [3, 4, 5, 6]);
    }
    {
        let (left, _right) = v.split_at(6);
        assert!(left == [1, 2, 3, 4, 5, 6]);
        // assert!(right == []);
    }
    //split_at_mut
    let mut v = [1, 0, 3, 0, 5, 6];
    {
        let (left, right) = v.split_at_mut(2);
        assert!(left == [1, 0]);
        assert!(right == [3, 0, 5, 6]);
        left[1] = 2;
        right[1] = 4;
    }
    assert!(v == [1, 2, 3, 4, 5, 6]);
    //split_first
    let x = &[0, 1, 2];
    if let Some((first, elements)) = x.split_first() {
        assert_eq!(first, &0);
        assert_eq!(elements, &[1, 2]);
    }
    //split_first_mut
    let x = &mut [0, 1, 2];
    if let Some((first, elements)) = x.split_first_mut() {
        *first = 3;
        elements[0] = 4;
        elements[1] = 5;
    }
    assert_eq!(x, &[3, 4, 5]);
    //split_last
    let x = &[0, 1, 2];
    if let Some((last, elements)) = x.split_last() {
        assert_eq!(last, &2);
        assert_eq!(elements, &[0, 1]);
    }
    //split_last_mut
    let x = &mut [0, 1, 2];
    if let Some((last, elements)) = x.split_last_mut() {
        *last = 3;
        elements[0] = 4;
        elements[1] = 5;
    }
    assert_eq!(x, &[4, 5, 3]);
    //split_mut
    let mut v = [10, 40, 30, 20, 60, 50];
    for group in v.split_mut(|num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(v, [1, 40, 30, 1, 60, 1]);
    //rsplitn_mut
    let mut s = [10, 40, 30, 20, 60, 50];
    for group in s.rsplitn_mut(2, |num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(s, [1, 40, 30, 20, 60, 1]);
    //sort
    let mut v = [-5, 4, 1, -3, 2];
    v.sort();
    assert!(v == [-5, -3, 1, 2, 4]);
    //sort_by
    let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];
    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0]);
    //sort_by_cached_key
    let mut v = [-5i32, 4, 32, -3, 2];
    v.sort_by_cached_key(|k| k.to_string());
    assert!(v == [-3, -5, 2, 32, 4]);
    //sort_by_key
    let mut v = [-5i32, 4, 1, -3, 2];
    v.sort_by_key(|k| k.abs());
    assert!(v == [1, 2, -3, 4, -5]);
    //splitn拆分n个
    let v = [10, 40, 30, 20, 60, 50];
    for group in v.splitn(2, |num| *num % 3 == 0) {
        println!("{:?}", group);//[10, 40], [20, 60, 50]
    }
    //splitn_mut
    let mut v = [10, 40, 30, 20, 60, 50];
    for group in v.splitn_mut(2, |num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(v, [1, 40, 30, 1, 60, 50]);
    //swap替换下标元素，这里数组和Vec都可以
    let mut v = ["a", "b", "c", "d"];
    v.swap(1, 3);
    assert!(v == ["a", "d", "c", "b"]);
    //swap_with_slice
    let mut slice1 = [0, 0];
    let mut slice2 = [1, 2, 3, 4];
    slice1.swap_with_slice(&mut slice2[2..]);
    assert_eq!(slice1, [3, 4]);
    assert_eq!(slice2, [1, 2, 0, 0]);
    //to_vec
    let s = [10, 40, 30];
    let _x = s.to_vec();
// Here, `s` and `x` can be modified independently.
    //windows
    let slice = ['r', 'u', 's', 't'];
    let mut iter = slice.windows(2);
    assert_eq!(iter.next().unwrap(), &['r', 'u']);
    assert_eq!(iter.next().unwrap(), &['u', 's']);
    assert_eq!(iter.next().unwrap(), &['s', 't']);
    assert!(iter.next().is_none());
    let slice = ['f', 'o', 'o'];
    let mut iter = slice.windows(4);
    assert!(iter.next().is_none());
}