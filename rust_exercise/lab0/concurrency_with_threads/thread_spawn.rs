/*
一段创建多线程的代码，thread::spawn() 可以创建新的线程。
1. 创建 1 个线程，该线程打印 1 - 10, 睡眠 10 s
2. main 主线程睡眠 5 s
观察到主线程执行完之后，并不会等待第一个 for 循环中的线程全部创建并睡眠结束。
*/
use std::thread;
use std::time::Duration;

fn main(){
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi from the spawned thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hello from main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}