mod lib;

use rand::Rng;

fn main() {
    lib::development::dev::add_task();
    println!("r : {:?}", rand::thread_rng().gen_range(1..100));
    println!("HI")
}
