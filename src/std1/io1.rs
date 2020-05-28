use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;

//读文件
pub fn i_t1() ->io::Result<()>{
    let mut f = File::open("asset/foo.txt")?;//文件在项目根部
    let mut buffer = [0; 10];
    let n = f.read(&mut buffer)?;
    println!("The bytes: {:?}", &buffer[..n]);//The bytes: [231, 156, 159, 231, 154, 132, 231, 137, 155, 231]
    println!("the String: {}",String::from_utf8_lossy(&buffer));//真的牛�
    Ok(())
}

//Seek跳转位置读
pub fn i_t2() ->io::Result<()>{
    let mut f = File::open("asset/foo.txt")?;
    let mut buffer = [0; 10];
    //跳到最后十个字节
    f.seek(SeekFrom::End(-10))?;
    let n = f.read(&mut buffer)?;
    println!("The bytes: {:?}", &buffer[..n]);
    println!("the String: {}",String::from_utf8_lossy(&buffer));//�狡辩。
    Ok(())
}

//BufReader直接读成String
pub fn i_t3() ->io::Result<()>{
    let f = File::open("asset/foo.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    //读一行到缓存
    reader.read_line(&mut buffer)?;
    println!("{}", buffer);
    let f = File::open("asset/foo.txt")?;
    let reader = BufReader::new(f);
    for line in reader.lines() {//一行一行全部打印出来
        println!("{}", line?);
    }
    Ok(())
}

//BufWriter
pub fn i_t4() ->io::Result<()>{
    let f = File::create("asset/write.txt")?;
    {
        let mut writer = BufWriter::new(f);
        // write a byte to the buffer
        writer.write(&[42])?;
    } // the buffer is flushed once writer goes out of scope
    Ok(())
}

//stdin
pub fn i_t5() ->io::Result<()>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!("You typed: {}", input.trim());
    Ok(())
}

//stdout
pub fn i_t6() ->io::Result<()>{
    //lock返回一个写保护，在函数结束后自动解锁
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(b"hello world")?;
    Ok(())
}

//stderr
pub fn i_t7() ->io::Result<()>{
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    handle.write_all(b"hello world")?;
    Ok(())
}