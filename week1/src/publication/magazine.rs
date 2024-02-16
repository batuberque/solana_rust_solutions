pub struct Magazine {
    pub title: String,
    pub issue: u32,
    pub topic: String,
}

impl Magazine {
    pub fn new(title: String, issue: u32, topic: String) -> Self {
        Self {
            title,
            issue,
            topic,
        }
    }
}
