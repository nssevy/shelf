use crate::pages::{BookId, book_detail};
use topcoat::{
    Result,
    router::href,
    view::{View, class, component, view},
};

#[component]
pub async fn book_card(
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
