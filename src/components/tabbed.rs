struct Tabbed {
    titles: Vec<String>,
    contents: Vec<u8>,
    selected_idx: usize,
}

impl Tabbed {
    pub fn new() -> Self {
        Self {
            titles: Vec::new(),
            contents: Vec::new(),
            selected_idx: 0,
        }
    }

    pub fn add(&mut self, title: String, content: u8) {
        self.titles.push(title);
        self.contents.push(content);
    }

    pub fn select(&mut self, idx: usize) {
        if idx < self.titles.len() {
            self.selected_idx = idx;
        }
    }
}
