
//Vec基本，Vec和String非常类似
pub fn vec_test1(){
    //基本使用
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);
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
}

//Vec
pub fn vec_test2(){
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
    //drain也是修改两者，范围参数
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