use chrono::NaiveDate;

#[derive(Debug, Default)]
pub struct BlogPost {
    pub title: String,
    pub content: String,
    pub author: String,
    pub published_date: Option<NaiveDate>,
}

impl BlogPost {
    pub fn new(title: &str, content: &str, author: &str) -> Self {
        Self {
            title: title.to_owned(),
            content: content.to_owned(),
            author: author.to_owned(),
            published_date: None,
        }
    }

    pub fn publish(&mut self, date: NaiveDate) {
        self.published_date = Some(date);
    }
}
