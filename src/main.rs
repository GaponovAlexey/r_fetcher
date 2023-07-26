fn main() {
    external::handler()
}

fn on_s() {
    println!("")
}
fn on_f() {}

mod external {
    pub fn handler(on_s: fn(), on_f: fn()) {
        let res = do_h();
        if res == true {
            on_s()
        } else {
            on_f()
        }
    }
    fn do_h() -> bool {
        false
    }
}
