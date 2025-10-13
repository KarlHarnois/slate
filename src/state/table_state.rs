pub struct TableState {
    pub title: String,
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub focused: bool,
}

impl TableState {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            header: Vec::new(),
            rows: Vec::new(),
            focused: false,
        }
    }
}

