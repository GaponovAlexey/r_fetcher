use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let n = "said_man".to_string();
    let x = "4epa".to_string();
    let y = MyBox::new(&n);
    *n = &x;
    hello(&y);
    hello(&n)
}

fn hello(name: &str) {
    println!("{:?}", name)
}
