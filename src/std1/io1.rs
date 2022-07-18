use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write},
};

#[allow(dead_code)]
pub fn i1() -> io::Result<()> {
    //read
    let mut f = File::open("asset/foo.txt")?; //文件在项目根部
    let mut buffer = [0; 10];
    let n = f.read(&mut buffer)?;
    println!("The bytes: {:?}", &buffer[..n]);
    println!("the String: {}", String::from_utf8_lossy(&buffer));
    //seek
    f.seek(SeekFrom::End(-10))?;
    let n = f.read(&mut buffer)?;
    println!("The bytes: {:?}", &buffer[..n]);
    println!("the String: {}", String::from_utf8_lossy(&buffer));
    //BufReader
    let f = File::open("asset/foo.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    println!("{}", buffer);
    let f = File::open("asset/foo.txt")?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        //一行一行全部打印出来
        println!("{}", line?);
    }
    //BufWriter
    let f = File::create("asset/write.txt")?;
    {
        let mut writer = BufWriter::new(f);
        writer.write(&[42])?;
    }
    Ok(())
}
