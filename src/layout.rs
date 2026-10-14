use crate::pages::{about, home};
use topcoat::{
    Result,
    font::fontsource::fontsource_font,
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
                <link rel="stylesheet" href=(topcoat::tailwind::stylesheet!())>
                topcoat::font::link(font: fontsource_font!(GEIST))
            </head>
            <body>
                <header>
                    <nav>
                        <a href=(href!(home))>"Home"</a>
                        <a href=(href!(about))>"about"</a>
                    </nav>
                </header>
                <main class="max-w-6xl mx-auto px-6 py-10">(slot)</main>
                <footer></footer>
            </body>
        </html>
    })
}
