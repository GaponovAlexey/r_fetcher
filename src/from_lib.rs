pub struct Ex {
  pub mess: String,
}

impl Ex {
  pub fn new() -> Ex {
      Ex { mess: "Hi from lib".to_string() }
  }
  pub fn change_name(name: &str) -> String {
      format!("hi my dia friend: {}", name.to_string())
  }
}
