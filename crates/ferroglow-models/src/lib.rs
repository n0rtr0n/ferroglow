use serde::Serialize;

#[derive(Serialize)]
pub struct Pattern {
    name: String,
}


impl Pattern {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
