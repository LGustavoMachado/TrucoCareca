pub struct Player {
  pub name: String,
  pub is_ready: bool
}

impl Player {
  pub fn new(name: String, is_ready: bool) -> Self { 
    Self { name, is_ready }
  }
}