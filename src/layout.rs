use crate::pages::{about, home};
use topcoat::{
    Result,
    router::{Slot, href, layout},
    view::{View, view},
};

#[layout("/")]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
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
