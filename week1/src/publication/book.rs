pub struct Book {
    pub title: String,
    pub author: String,
    pub page_count: u32,
}

impl Book {
    pub fn new(title: String, author: String, page_count: u32) -> Self {
        Self {
            title,
            author,
            page_count,
        }
    }
}
