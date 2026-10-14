use crate::books::Book;
use crate::components::book_card;

use topcoat::{
    Result,
    context::Cx,
    router::{error::RouterErrorExt, href, page, path_param},
    view::{View, view},
};

#[page("/")]
pub async fn home() -> Result<impl View> {
    let livres: Vec<Book> = Book::all();
    Ok(view! {
        <h1>
            "Mes livres ("
            (livres.len())
            " books)"
        </h1>
        for livre in livres {
            book_card(
                id: livre.id,
                title: &livre.title,
                author: &livre.author,
                year: livre.year,
                finished: livre.finished
            )
        }
    })
}

#[page("/about")]
pub async fn about() -> Result<impl View> {
    let prenom: String = "Sevy".into();
    Ok(view! {
        <h2>
            "Bonjours je me présente "
            (prenom)
        </h2>
    })
}

path_param!(pub book_id: u32, error=not_found);
#[page("/books/{book_id}")]
pub async fn book_detail(cx: &Cx) -> Result<impl View> {
    let id: &u32 = path_param::<BookId>(cx)?;
    let book: Book = Book::find(*id).ok_or_not_found()?;
    Ok(view! {
        <h1>(book.title)</h1>
        <p>
            (book.author)
            "|"
            (book.year)
        </p>
        if book.finished {
            <p>"Livre terminé"</p>
        } else {
            <p>"Pas encore fini de lire"</p>
        }
        <a href=(href!(home))>"Retour"</a>
    })
}
