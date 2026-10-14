mod books;
use crate::books::Book;

use topcoat::{
    Result,
    context::Cx,
    router::{
        Router, RouterBuilderDiscoverExt, Slot, error::RouterErrorExt, href, layout, page,
        path_param,
    },
    view::{View, class, component, view},
};

#[tokio::main]
async fn main() {
    topcoat::start(Router::builder().discover().build())
        .await
        .unwrap()
}

#[layout("/")]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    //let link_url: Vec![""]
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Shelf"</title>
                topcoat::dev::script()
            </head>
            <body>
                <header>
                    <nav>
                        <a href=(href!(home))>"Home"</a>
                        <a href=(href!(about))>"about"</a>
                    </nav>
                </header>
                <main>(slot)</main>
                <footer></footer>
            </body>
        </html>
    })
}

#[page("/")]
async fn home() -> Result<impl View> {
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
async fn about() -> Result<impl View> {
    let prenom: String = "Sevy".into();
    Ok(view! {
        <h2>
            "Bonjours je me présente "
            (prenom)
        </h2>
    })
}

path_param!(book_id: u32, error=not_found);
#[page("/books/{book_id}")]
async fn book_detail(cx: &Cx) -> Result<impl View> {
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

#[component]
async fn book_card(
    id: u32,
    title: &str,
    author: &str,
    year: u32,
    finished: bool,
) -> Result<impl View> {
    Ok(view! {
        <article>
            <h2><a href=(href!(book_detail, BookId(id)))>(title)</a></h2>
            <p>
                "L'auteur est : "
                (author)
                ", sorti en "
                (year)
            </p>
            <span class=(class!("badge", "done" if finished))>
                if finished {
                    "Lu"
                } else {
                    "En cours"
                }
            </span>
        </article>
    })
}

/*#[component]
async fn hello(name: &str) -> Result<impl View> {
    let genres: Vec<&str> = vec![
        "Bande dessinée",
        "Roman sentimental",
        "Littérature de l'imaginaire",
    ];
    let is_logged_in: bool = false;
    let about: &str = "A propos de nous";

    Ok(view! {
        <h1>
            "Salut "
            (name)
        </h1>
        <ul>
            for genre in genres {
                <li>(genre)</li>
            }
        </ul>
        if is_logged_in {
            <p>"Connecté"</p>
        } else {
        <a
            href="/login"
            if is_logged_in {
                class="active"
                aria-current="page"
            }
        >
            "Se connecter"
        </a>}
        <a href="/about" title=(about)>"About"</a>
    })
}*/
