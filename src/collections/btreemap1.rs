use std::collections::BTreeMap;
use std::ops::Bound::Included;

//BTreeMap
#[allow(dead_code)]
pub fn b1(){
    let mut m1=BTreeMap::new();
    m1.insert(1,"a");
    println!("{}",m1.len());//1
    println!("{:?} {:?}",m1.get(&1),m1.get_key_value(&1));//Some("a") Some((1, "a"))
    if let Some(x)=m1.get_mut(&1){
        *x="nmsl";
    }
    println!("{:?}",m1);//{1: "nmsl"}
    println!("{}",m1.contains_key(&1));//true
    m1.clear();
    println!("{}",m1.is_empty());//true
    m1.insert(2,"b");
    m1.insert(3,"c");
    println!("{:?} {:?}",m1.remove(&2),m1.remove_entry(&3));//Some("b") Some((3, "c"))
    println!("{}",m1.is_empty());//true
    let mut m2 = BTreeMap::new();
    m2.insert(1, "a");
    m2.insert(2, "b");
    m2.insert(3, "c");
    let mut m3 = BTreeMap::new();
    m3.insert(3, "d");
    m3.insert(4, "e");
    m3.insert(5, "f");
    m2.append(&mut m3);//append会改变两个map，有相同的key，后者会覆盖前者
    println!("{:?} {:?}",m2,m3);//{1: "a", 2: "b", 3: "d", 4: "e", 5: "f"} {}
    //range是BTreeMap的特有方法
    let mut m4 = BTreeMap::new();
    m4.insert(3, "a");
    m4.insert(5, "b");
    m4.insert(8, "c");
    for (&key, &value) in m4.range((Included(&4), Included(&8))) {
        println!("{}: {}", key, value);
    }
    // 5: b
    // 8: c
    println!("{:?}",m4.range(4..).next());//Some((5, "b"))
    //range_mut
    let mut m5: BTreeMap<&str, i32> = ["Alice", "Bob", "Carol", "Cheryl"]
        .iter()
        .map(|&s| (s, 0))
        .collect();
    for (_, balance) in m5.range_mut("B".."Cheryl") {
        *balance += 100;
    }
    for (name, balance) in &m5 {
        println!("{} => {}", name, balance);
    }
    // Alice => 0
    // Bob => 100
    // Carol => 100
    // Cheryl => 0
    //entry
    let mut m6: BTreeMap<&str, usize> = BTreeMap::new();
    for x in vec!["a","b","a","c","a","b"] {
        *m6.entry(x).or_insert(0) += 1;
    }
    assert_eq!(m6["a"], 3);
    //split_off也是BTreeMap独有的方法
    let mut m7 = BTreeMap::new();
    m7.insert(1, "a");
    m7.insert(2, "b");
    m7.insert(3, "c");
    m7.insert(17, "d");
    m7.insert(41, "e");
    let m8 = m7.split_off(&3);
    println!("{:?} {:?}",m7,m8);//{1: "a", 2: "b"} {3: "c", 17: "d", 41: "e"}
    //iter迭代器
    let mut m9 = BTreeMap::new();
    m9.insert(13, "c");
    m9.insert(22, "b");
    m9.insert(100, "a");
    for (key, value) in m9.iter() {
        println!("{}: {}", key, value);
    }
    //BTreeMap的key值是排序的！！
    // 13: c
    // 22: b
    // 100: a
    let (first_key, first_value) = m9.iter().next().unwrap();
    println!("{}: {}",first_key,first_value);//13: c
    let mut m10 = BTreeMap::new();
    m10.insert("a", 1);
    m10.insert("b", 2);
    m10.insert("c", 3);
    for (key, value) in m10.iter_mut() {
        if key != &"a" {
            *value += 10;
        }
    }
    println!("{:?}",m10);//{"a": 1, "b": 12, "c": 13}
    //keys、values
    let mut m11 = BTreeMap::new();
    m11.insert(2, "b");
    m11.insert(1, "a");
    let keys: Vec<_> = m11.keys().cloned().collect();
    assert_eq!(keys, [1, 2]);
    let values: Vec<&str> = m11.values().cloned().collect();
    assert_eq!(values, ["a", "b"]);//这里必须是a、b，因为会按照key的大小排序
    let mut m12 = BTreeMap::new();
    m12.insert(1, String::from("hello"));
    m12.insert(2, String::from("goodbye"));
    for value in m12.values_mut() {
        value.push_str("!");
    }
    let values: Vec<String> = m12.values().cloned().collect();
    assert_eq!(values, [String::from("hello!"),
        String::from("goodbye!")]);
}