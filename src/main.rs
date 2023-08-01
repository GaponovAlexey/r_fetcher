use std::{ sync::mpsc::{ Receiver, Sender, self }, thread, time::Duration };

fn main() {
    let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

    let sed_clone = sender.clone();
    thread::spawn(move || {
        let vals = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string()
        ];
        for val in vals {
            sender.send(val).expect("err in val");
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string()
        ];
        for val in vals {
            sed_clone.send(val).expect("err in val");
            thread::sleep(Duration::from_secs(1));
        }
    });

    for rec in receiver {
        println!("{:?}", rec);
    }
}
