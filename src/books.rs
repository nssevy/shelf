#[derive(Clone, serde::Serialize)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub year: u32,
    pub finished: bool,
}
impl Book {
    pub fn all() -> Vec<Book> {
        vec![
            Book {
                id: 1,
                title: "1984".into(),
                author: "George Orwell".into(),
                year: 1949,
                finished: true,
            },
            Book {
                id: 2,
                title: "L'Étranger".into(),
                author: "Albert Camus".into(),
                year: 1942,
                finished: false,
            },
            Book {
                id: 3,
                title: "Le Meilleur des mondes".into(),
                author: "Aldous Huxley".into(),
                year: 1931,
                finished: true,
            },
        ]
    }
    pub fn find(id: u32) -> Option<Book> {
        Book::all().into_iter().find(|book| book.id == id)
    }
}
