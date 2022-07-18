

#[allow(dead_code)]
pub fn h1(){
    //new,len,capacity
    let map: HashMap<&str, i32> = HashMap::new();
    assert_eq!(map.len(),0);
    assert_eq!(map.capacity(),0);
    //clear
    let mut a = HashMap::new();
    a.insert(1, "a");
    a.clear();
    assert!(a.is_empty());
    //contains_key
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.contains_key(&1), true);
    assert_eq!(map.contains_key(&2), false);
    //drain清空map，返回一个迭代器存储所有的键值对
    let mut a = HashMap::new();
    a.insert(1, "a");
    a.insert(2, "b");
    for (k, v) in a.drain().take(1) {
        assert!(k == 1 || k == 2);
        assert!(v == "a" || v == "b");
    }
    assert!(a.is_empty());
    //entry根据key返回键值对
    let mut letters = HashMap::new();
    for ch in "a short treatise on fungi".chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }
    assert_eq!(letters[&'s'], 2);//map的值是单个字符在字符串里的出现次数
    assert_eq!(letters[&'t'], 3);
    assert_eq!(letters[&'u'], 1);
    assert_eq!(letters.get(&'y'), None);
    //get参数和返回值都是引用
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&2), None);
    //get_key_value
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
    assert_eq!(map.get_key_value(&2), None);
    //get_mut返回可变引用
    let mut map = HashMap::new();
    map.insert(1, "a");
    if let Some(x) = map.get_mut(&1) {
        *x = "b";
    }
    assert_eq!(map[&1], "b");
    //hasher
    use std::collections::HashMap;
    use std::collections::hash_map::RandomState;
    let hasher = RandomState::new();
    let map: HashMap<i32, i32> = HashMap::with_hasher(hasher);
    let _hasher: &RandomState = map.hasher();
    //insert如果新插入值则返回None，如果key已经存在则返回旧值
    let mut map = HashMap::new();
    assert_eq!(map.insert(37, "a"), None);
    assert_eq!(map.is_empty(), false);
    map.insert(37, "b");
    assert_eq!(map.insert(37, "c"), Some("b"));
    assert_eq!(map[&37], "c");
    //is_empty
    let mut a = HashMap::new();
    assert!(a.is_empty());
    a.insert(1, "a");
    assert!(!a.is_empty());
    //iter迭代元素是(&'a K, &'a V)
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    for (key, val) in map.iter() {
        println!("key: {} val: {}", key, val);
    }
    //iter_mut迭代元素是(&'a K, &'a mut V)
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    for (_, val) in map.iter_mut() {
        *val *= 2;
    }
    for (key, val) in &map {
        println!("key: {} val: {}", key, val);
    }
    //keys迭代元素是&'a K
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    for key in map.keys() {
        println!("{}", key);
    }
    //remove删除值成功则返回旧值
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.remove(&1), Some("a"));
    assert_eq!(map.remove(&1), None);
    //remove_entry删除值成功则返回旧的entry
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.remove_entry(&1), Some((1, "a")));
    assert_eq!(map.remove(&1), None);
    //reserve
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.reserve(10);
    assert!(map.capacity()>=10);
    //retain只保留符合函数条件的键值对
    let mut map: HashMap<i32, i32> = (0..8).map(|x|(x, x*10)).collect();
    map.retain(|&k, _| k % 3 == 0);
    assert_eq!(map.len(), 3);
    //shrink_to_fit缩减capacity
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.capacity() >= 100);
    map.shrink_to_fit();
    assert!(map.capacity() >= 2);
    //values迭代元素是&'a V
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    for val in map.values() {
        println!("{}", val);
    }
    //values_mut迭代元素是&'a mut V
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    for val in map.values_mut() {
        *val = *val + 10;
    }
    for val in map.values() {
        println!("{}", val);
    }
    //with_capacity_and_hasher指定capacity和hasher，也可以分开指定
    let s = RandomState::new();
    let mut map = HashMap::with_capacity_and_hasher(10, s);
    map.insert(1, 2);
    let _map: HashMap<&str, i32> = HashMap::with_capacity(10);
    let s = RandomState::new();
    let mut map = HashMap::with_hasher(s);
    map.insert(1, 2);
}