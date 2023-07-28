use std::ops::Add;

struct MySmartPointer<T: Add<Output = T> + Copy + From<i32>>(T);

impl<T: Add<Output = T> + Copy + From<i32>> MySmartPointer<T> {
    fn new(x: T) -> MySmartPointer<T> {
        MySmartPointer(x)
    }

    fn add(&self) -> T {
        self.0 + T::from(344)
    }
}

fn main() {
    let number = 123;
    let one = Box::new(number);
    println!("{:?}", one);
    let two = Box::new(one);
    println!("{:?}", two);
    let three = Box::new(two);
    println!("{:?}", three);

    let my_box = MySmartPointer::new(number);
    println!("{:?}", my_box.add());
}
