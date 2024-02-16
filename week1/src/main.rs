mod publication;

use publication::book::Book;
use publication::magazine::Magazine;
use publication::Publication;

fn main() {
    let publications = vec![
        Publication::Book(Book::new(
            "Selamın Aleyküm".to_string(),
            "Aleyküm Selam".to_string(),
            626,
        )),
        Publication::Magazine(Magazine::new(
            "The Rust Times".to_string(),
            22,
            "System Programming".to_string(),
        )),
    ];

    publication::print_publications(publications);
}
