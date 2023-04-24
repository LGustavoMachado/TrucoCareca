#[derive(Debug)]
pub struct Player {
    name: String,
    is_ready: bool,
}

impl Player {
    pub fn new(name: String, is_ready: bool) -> Self {
        Self { name, is_ready }
    }

    pub fn update(&mut self, name: String, is_ready: bool) {
        self.name = name;
        self.is_ready = is_ready;
    }

    pub fn is_ready(&self) -> bool {
        self.is_ready
    }
}
