use r_fetcher::add_one;
fn main() {
    let x = add_one(5);
    println!("{:?}", x);
    let answer = add_one(5);
    assert_eq!(7, answer)
}
