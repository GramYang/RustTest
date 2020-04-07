pub fn fn_test1() {
    let a = [1, 2, 3, 4, 8, 9];
    println!("There is 7 in the array: {}", find(7, &a)); //false
    println!("There is 8 in the array: {}", find(8, &a)); //true
}

fn find(n: i32, a: &[i32]) -> bool {
    for i in a {
        if *i == n {
            return true;
        }
    }
    false
}