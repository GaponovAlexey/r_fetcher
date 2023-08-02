use std::sync::{ Arc, RwLock };
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(0));

    let mut handles = vec![];

    // Спавним 10 потоков, которые увеличивают значение
    for _ in 0..10 {
        let data = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let mut num = data.write().unwrap();
            *num += 10;
        });

        handles.push(handle);
    }

    // Спавним 10 потоков, которые читают значение
    for _ in 0..10 {
        let data = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let num = data.read().unwrap();

            println!("r: {}", *num);
        });

        handles.push(handle);
    }

    // Дожидаемся завершения всех потоков
    for handle in handles {
        println!("{:?}", handle);
        handle.join().unwrap();
    }
}
