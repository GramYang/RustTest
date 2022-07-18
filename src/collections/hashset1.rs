use std::collections::HashSet;


//HashSet
#[allow(dead_code)]
pub fn h1(){
    let mut s1: HashSet<&str> = HashSet::with_capacity(10);
    println!("{}",s1.capacity());//14
    s1.insert("a");
    s1.insert("b");
    for x in s1.iter() {
        println!("{}", x);
    }
    println!("{}",s1.len());//2
    for i in s1.drain() {
        println!("{}", i);
    }
    println!("{}",s1.is_empty());//true
    let mut s2: HashSet<i32> = HashSet::new();
    s2.reserve(10);
    println!("{}",s2.capacity());//14
    let mut s3 = HashSet::with_capacity(100);
    s3.insert(1);
    s3.insert(2);
    println!("{}",s3.capacity());//114
    s3.shrink_to_fit();
    println!("{}",s3.capacity());//3
    //difference是HashSet特有方法
    let s4: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let s5: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();
    println!("{:?} {:?}",s4,s5);//{2, 3, 1} {3, 2, 4}
    let s6:HashSet<_>=s4.difference(&s5).collect();
    let s7:HashSet<_>=s5.difference(&s4).collect();
    println!("{:?} {:?}",s6,s7);//{1} {4}
    //symmetric_difference和difference不同，主从顺序的结果都是一样的
    let s8:HashSet<_>=s4.symmetric_difference(&s5).collect();
    let s9:HashSet<_>=s5.symmetric_difference(&s4).collect();
    println!("{:?} {:?}",s8,s9);//{4, 1} {1, 4}
    //intersection是提取相同的子集
    let s10:HashSet<_>=s4.intersection(&s5).collect();
    println!("{:?}",s10);//{3, 2}
    //union求并集
    let s11: HashSet<_> = s4.union(&s5).collect();
    println!("{:?}",s11);//{1, 3, 2, 4}
    //is_disjoint没有相同元素就返回true
    let s12: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let mut s13 = HashSet::new();
    println!("{}",s12.is_disjoint(&s13));//true
    s13.insert(4);
    println!("{}",s12.is_disjoint(&s13));//true
    s13.insert(1);
    println!("{}",s12.is_disjoint(&s13));//false
    //is_subset如果是另一个集合的子集就返回true
    let mut s14 = HashSet::new();
    println!("{}",s14.is_subset(&s12));//true
    s14.insert(2);
    println!("{}",s14.is_subset(&s12));//true
    s14.insert(4);
    println!("{}",s14.is_subset(&s12));//false
    //is_superset如果是另一个集合的父集就返回true
    let s15: HashSet<_> = [1, 2].iter().cloned().collect();
    let mut s16 = HashSet::new();
    println!("{}",s16.is_superset(&s15));//false
    s16.insert(0);
    s16.insert(1);
    println!("{}",s16.is_superset(&s15));//false
    s16.insert(2);
    println!("{}",s16.is_superset(&s15));//true
}