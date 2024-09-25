/*
练习 3. 在 Linux 环境下编写一个可以睡眠 5 秒后打印一个字符串，并把字符串内容存入一个文件中的应用程序 A。
*/
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::Write;

fn main(){
    for i in 0..5 {
        println!("Main thread at point {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    let content = "This is a string to be written! for hands_on_exer3!";
    let filename = "exer3.txt";

    let mut file = match File::create(filename){
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create file {}", e);
            return;
        },
    };

    match writeln!(file, "{}", content){
        Ok(()) => println!("Successfuly written string to file!"),
        Err(e) => {
            eprintln!("Failed to write to file {}", e);
            return;
        },
    };
}