use crate::books::Book;
use topcoat::{
    Result,
    router::{content::Json, route},
};

#[route(GET "/api/health")]
async fn health() -> Result<&'static str> {
    Ok("ok")
}

#[route(GET "/api/books")]
async fn api_books() -> Result<Json<Vec<Book>>> {
    Ok(Json(Book::all()))
}
