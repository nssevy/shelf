use topcoat::{
    Result,
    router::{Router, RouterBuilderDiscoverExt, page},
    view::{View, component, view},
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
            <body>hello(name: "Sevy")</body>
        </html>
    })
}

#[component]
async fn hello(name: &str) -> Result<impl View> {
    let genres: Vec<&str> = vec![
        "Bande dessinée",
        "Roman sentimental",
        "Littérature de l'imaginaire",
    ];
    let is_logged_in: bool = false;
    let about: &str = "A propos de nous";

    Ok(view! {
        <h1>"Salut "(name)</h1>
        <ul>
            for genre in genres {
                <li>(genre)</li>
            }
        </ul>
        if is_logged_in {
            <p>"Connecté"</p>
        } else {
            <p><a href="/login">"Se connecter"</a></p>
        }
        <a href="/about" title=(about)>"About"</a>
    })
}
