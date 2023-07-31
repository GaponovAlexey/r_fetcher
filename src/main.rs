// use r_fetcher::from_lib::*;

use r_fetcher::{ ex::Ex, testsend::Testsend };

fn main() {
    let pr = Ex::new();
    println!("{:?}", pr.mess);

    let n1 = Ex::change_name("Serg");
    println!("{:?}", n1);
    let r1 = Testsend("str1");
    println!("{:?}", r1.trim());
    println!("HI");
}
