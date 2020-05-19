use chrono::prelude::*;

//chrono使用
pub fn c_t1(){
    let d= Local::now();
    println!("{}\n{}\n{}\n{}\n",d.to_string(),d.to_rfc2822(),d.to_rfc3339(),format!("{:?}",d));
    // 2020-05-19 07:05:10.972596900 UTC
    // Tue, 19 May 2020 07:05:10 +0000
    // 2020-05-19T07:05:10.972596900+00:00
    // 2020-05-19T07:05:10.972596900Z
    println!("{}\n{}\n{}\n",d.format("%Y-%m-%d %H:%M:%S"), d.format("%a %b %e %T %Y"),d.format("%c"));
    // 2020-05-19 07:05:10
    // Tue May 19 07:05:10 2020
    // Tue May 19 07:05:10 2020
    println!("{} {} {} {} {} {}",d.year(),d.month(),d.day(),d.hour(),d.minute(),d.second());
    println!("{} {} {}",d.weekday(),d.weekday().number_from_monday(),d.ordinal());
    // 2020 5 19 7 5 10
    // Tue 2 140
}