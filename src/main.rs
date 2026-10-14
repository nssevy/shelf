use topcoat::{
    Result,
    router::{Router, RouterBuilderDiscoverExt, page},
    view::{View, class, component, view},
};
#[tokio::main]
async fn main() {
    topcoat::start(Router::builder().discover().build())
        .await
        .unwrap()
}

#[page("/")]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Shelf"</title>
                topcoat::dev::script()
            </head>
            <body>
                book_card(
                    title: "1984",
                    author: "George Orwell",
                    years: 1949,
                    finished: true
                )
                book_card(
                    title: "L'Étranger",
                    author: "Albert Camus",
                    years: 1942,
                    finished: false
                )
                book_card(
                    title: "Le Meilleur des mondes",
                    author: "Aldous Huxley",
                    years: 1931,
                    finished: true
                )
            </body>
        </html>
    })
}

#[component]
async fn book_card(title: &str, author: &str, years: u32, finished: bool) -> Result<impl View> {
    Ok(view! {
        <article>
            <h2>(title)</h2>
            <p>
                "L'auteur est : "
                (author)
                ", sorti en "
                (years)
            </p>
            <span
                class = (class!("badge", "done" if finished ))
            >
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
