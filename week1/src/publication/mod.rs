pub mod book;
pub mod magazine;

pub enum Publication {
    Book(book::Book),
    Magazine(magazine::Magazine),
}

pub trait Printable {
    fn print(&self);
}

impl Printable for Publication {
    fn print(&self) {
        match self {
            Publication::Book(book) => {
                println!(
                    "Book: {} author: {}, {} pages",
                    book.title, book.author, book.page_count
                );
            }
            Publication::Magazine(magazine) => {
                println!(
                    "Magazine: {} - Issue: {}, Topic: {}",
                    magazine.title, magazine.issue, magazine.topic
                );
            }
        }
    }
}

pub fn print_publications(publications: Vec<Publication>) {
    for publication in publications {
        publication.print();
    }
}
