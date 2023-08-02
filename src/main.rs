use gotham::state::State;

const HELLO_WORLD: &str = "Hello World!";

pub fn say_hello(state: State) -> (State, &'static str) {
    (state, HELLO_WORLD)
}

pub fn main() {
    let addr = "127.0.0.1:3000";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello)).expect("err");
}
