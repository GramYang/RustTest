use std::io::BufRead;
use memchr;

#[allow(dead_code)]
pub fn t1(){
    let xml = r#"11<11>11>'""<>>""#;
    let mut xml1=xml.as_bytes();
    let n=xml1.fill_buf();
    let n1=n.unwrap();
    let n2=memchr::memchr(b'<',n1);
    let n3=n2.unwrap();
    println!("memchr {}",n3);
    let mut n4=memchr::memchr3_iter(b'>',b'\'',b'"',n1);
    loop{
        match n4.next(){
            Some(i)=> println!("memchr3_iter {}",i),
            None => break
        }
    }
}