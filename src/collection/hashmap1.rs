use std::collections::HashMap;

//hashmap使用
pub fn hm_test1() {
    let mut scores=HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);
    let teams=vec![String::from("Red"),String::from("White")];
    let values=vec![30,70];
    let map:HashMap<_,_>=teams.iter().zip(values.iter()).collect();
    let k=String::from("Red");
    println!("{}",map.get(&k).unwrap()); //30
    for (key, value) in &scores{
        println!("{}: {}",key,value);
    }
    println!("{:?}", scores); //{"Blue": 10, "Yellow": 50}
}