use std::{ fs::File, io::Read };

use serde_json::{ self, Value };

use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
struct Dev {
    name: String,
    age: i32,
    address: Address,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
}
fn main() {
    let mut file = File::open("src/data.json").expect("err open file");
    let mut contents = "".to_string();
    file.read_to_string(&mut contents).expect("err read");
    let deserialized: Dev = serde_json::from_str(&contents).expect("err serde");
    println!("{:?}", deserialized);
}
