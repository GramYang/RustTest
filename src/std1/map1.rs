use std::collections::HashMap;

//hashmap使用
pub fn hashmap1(){
    let mut book = HashMap::new();
    book.insert(1,10);
    book.insert(2,20);
    book.insert(3,30);
    book.insert(4,40);
    if !book.contains_key(&5){
        println!("键不包含5，map长度为{}",book.len());//键不包含5，map长度为4
    }
    book.remove(&4);
    let to_find=[2,3];
    for b in &to_find{
        match book.get(&b){
            Some(r)=>println!("{}: {}",b,r),
            None=>println!("map中没有{}",b)
        }
    }
    // 2: 20
    // 3: 30
    println!("map[1]的值为{}",book[&1]);//map[1]的值为10
    for (b, r) in &book{
        println!("{}: \"{}\"", b, r);
    }
    //3: "30"
    //2: "20"
    //1: "10"
    let mut player:HashMap<&str,i32>=HashMap::new();
    player.entry("health").or_insert(100);
    player.entry("defence").or_insert_with(|| random());
    let stat=player.entry("attack").or_insert(100);
    *stat +=random();
    for (b, r) in &player{
        println!("{}: {}", b, r);
    }
    // defence: 42
    // health: 100
    // attack: 142
    let mut vikings = HashMap::new();
    vikings.insert(Viking::new("Einar", "Norway"), 25);
    vikings.insert(Viking::new("Olaf", "Denmark"), 24);
    vikings.insert(Viking::new("Harald", "Iceland"), 12);
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
    // Viking { name: "Olaf", country: "Denmark" } has 24 hp
    // Viking { name: "Harald", country: "Iceland" } has 12 hp
    // Viking { name: "Einar", country: "Norway" } has 25 hp
    let timber: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
        .iter().cloned().collect();
    for (b, r) in &timber{
        println!("{}: {}", b, r);
    }
    // Iceland: 10
    // Denmark: 50
    // Norway: 100
}

fn random()->i32{
    42
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking { name: name.to_string(), country: country.to_string() }
    }
}