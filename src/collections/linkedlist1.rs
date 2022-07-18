use std::collections::LinkedList;

//LinkedList
#[allow(dead_code)]
pub fn l1(){
    //append、push_back、push_front
    let mut l1=LinkedList::<i32>::new();
    l1.push_back(1);
    l1.push_back(2);
    let mut l2=LinkedList::<i32>::new();
    l2.push_front(3);
    l2.push_front(4);
    l1.append(&mut l2);//append会改变两者的值
    println!("{:?} {:?}",l1,l2);//[1, 2, 4, 3] []
    //is_empty
    println!("{}",l2.is_empty());//true
    //len
    println!("{}",l1.len());//4
    //iter、iter_mut
    {
        let mut it1=l1.iter();
        println!("{:?} {:?} {:?} {:?} {:?}",it1.next(),it1.next(),it1.next(),it1.next(),it1.next());
    }
    //Some(1) Some(2) Some(4) Some(3) None
    for e in l1.iter_mut(){
        *e+=10;
    }
    println!("{:?}",l1);//[11, 12, 14, 13]
    //clear
    l1.clear();
    println!("{}",l1.is_empty());//true
    //contains、front、front_mut、back、back_mut
    let mut l3=LinkedList::new();
    l3.push_back(21);
    l3.push_back(22);
    l3.push_back(23);
    println!("{}",l3.contains(&22));//true
    println!("{:?}",l3.front());//Some(21)
    match l3.front_mut() {
        None => {},
        Some(x) => *x += 5,
    }
    println!("{:?}",l3.front());//Some(26)
    println!("{:?}",l3.back());//Some(23)
    match l3.back_mut() {
        None => {},
        Some(x) => *x += 5,
    }
    println!("{:?}",l3.back());//Some(28)
    //split_off
    let mut l4 = LinkedList::new();
    l4.push_front(1);
    l4.push_front(2);
    l4.push_front(3);
    let mut split = l4.split_off(2);
    assert_eq!(split.pop_front(), Some(1));
    assert_eq!(split.pop_front(), None);
}