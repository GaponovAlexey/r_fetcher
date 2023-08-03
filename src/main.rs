struct MyIter<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let elm = self.slice.get(0);
        if self.slice.len() > 0 {
            self.slice = &self.slice[1..];
        }
        elm
    }
}

fn main() {
    let col = vec![1, 2, 3, 4];
    let iter = MyIter { slice: &col[..] };
    for (i, e) in iter.enumerate() {
        print!("E = {:?} :", e);
        println!("ind :{:?}", i);
        assert_eq!(*e, col[i]);
    }
}
