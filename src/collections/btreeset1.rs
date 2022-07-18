use std::collections::BTreeSet;
use std::ops::Bound::Included;

//BTreeSet
#[allow(dead_code)]
pub fn b1(){
    //range
    let mut s1 = BTreeSet::new();
    s1.insert(3);
    s1.insert(5);
    s1.insert(8);
    for &elem in s1.range((Included(&4), Included(&8))) {
        println!("{}", elem);
    }
    println!("{:?}",s1.range(4..).next());//Some(5)
    //split_off
    let mut s2 = BTreeSet::new();
    s2.insert(1);
    s2.insert(2);
    s2.insert(3);
    s2.insert(17);
    s2.insert(41);
    let s3 = s2.split_off(&3);
    println!("{:?} {:?}",s2,s3);//{1, 2} {3, 17, 41}
}