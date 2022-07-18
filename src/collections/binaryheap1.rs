use std::collections::BinaryHeap;


//BinaryHeap
#[allow(dead_code)]
pub fn b1(){
    let mut h1 = BinaryHeap::with_capacity(10);
    println!("{}",h1.capacity());//10
    h1.push(4);
    println!("{:?}",h1.peek());//Some(4)
    {
        let mut val = h1.peek_mut().unwrap();
        *val += 10;
    }
    println!("{:?}",h1.peek());//Some(14)
    h1.pop();
    println!("{}",h1.is_empty());//true
    //drain
    let mut h2 = BinaryHeap::from(vec![1, 3]);
    for x in h2.drain() {
        println!("{}", x);
    }
    println!("{}",h2.is_empty());//true
    //into_vec
    let h3 = BinaryHeap::from(vec![1, 2, 3, 4, 5, 6, 7]);
    let v1 = h3.into_vec();
    println!("{:?}", v1);//[7, 5, 6, 4, 2, 1, 3]
    let mut h4 = BinaryHeap::from(vec![1, 2, 4, 5, 7]);
    h4.push(6);
    h4.push(3);
    let v2 = h4.into_sorted_vec();
    println!("{:?}",v2);//[1, 2, 3, 4, 5, 6, 7]
}