#[derive(Debug)]
pub struct TextInput {
    pub value: String,
    pub cursor: usize,
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
        }
    }

    pub fn insert(&mut self, char: char) {
        self.value.insert(self.cursor, char);
        self.cursor += 1;
    }
}
