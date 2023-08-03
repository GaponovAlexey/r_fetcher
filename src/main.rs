use r_fetcher::some_mod::*;
fn main() {
    let d = Dev::new("Rust".to_string());
    let mut project = Project {
        members: Vec::new(),
    };

    let qa = Qa {};
    project.members.push(Box::new(d));
    project.members.push(Box::new(qa));
    for i in project.members.iter() {
        i.do_task();
    }
}
