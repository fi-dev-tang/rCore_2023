/*
让创建出来的线程先完成打印工作，再执行 main 线程
*/
use std::thread;
use std::time::Duration;

fn main(){
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi from spawned thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("Hello from main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}