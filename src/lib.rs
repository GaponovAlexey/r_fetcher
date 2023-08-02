use std::{ thread::{ self, JoinHandle }, time::Duration };
use futures::join;

pub async fn task_print() {
    println!("print from future")
}

pub async fn eat() {
    for i in 0..5 {
        println!("eat -> = {:?}", thread::current().id());
        async_std::task::sleep(Duration::from_millis(1000)).await;
        println!("eat -> = {:?}", thread::current().id());
    }
}
pub async fn play() {
    for i in 0..5 {
        println!("play -> = {:?}", thread::current().id());
        async_std::task::sleep(Duration::from_millis(1000)).await;
        println!("play -> = {:?}", thread::current().id());
    }
}
pub async fn eat_play() {
    println!("Task eat and play");
    eat().await;
    play().await;
}

pub async fn eat_play_concurrently() {
    println!("from Concurrently");
    let f1 = eat();
    let f2 = play();
    join!(f1, f2);
}
