//项目：password_generator
//创建人：ColoFly
//创建日期：2024.04.08
//版本：v1.0

use rand::Rng;
use std::io;
use std::fs::File;
use std::path::Path;
use std::io::Write;

fn main() {
    let mut passwd_length = String::new();
    let mut rng = rand::thread_rng();
//提示信息
    println!("Please input passwd length: {}", passwd_length);
//捕获用户输入的密码长度；
    io::stdin()
        .read_line(&mut passwd_length)
        .expect("Failed to read line");
//判断输入的是否是数字
    let passwd_length: u8 = passwd_length
        .trim().parse().
        expect("Please input number.");
//定义密码取值范围,最多为256个字符
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
//通过上述随机生成复杂密码
    let password: String = (0.. passwd_length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

//定义密码保存的文件路径
    let path = Path::new("./passwd.txt");
    let display = path.display();

/* 
在程序所在目录下创建一个"passwd.txt"文件。
*/
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {:?}", display, why),
        Ok(file) => file,
    };

//将上述生成的密码"passwd"写入这个文件。
    match file.write_all(password.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}, {:?}", display, why)
        },
        Ok(_) => println!("Successfully worote to {}", display),
    }

}
