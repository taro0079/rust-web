use axum::{Router, routing, response::{IntoResponse} };

use crate::response;
use crate::views::home::Home;
use crate::views::partial::tweet::Tweet;


pub fn app() -> Router {
    Router::new().route("/", routing::get(get))
}

async fn get() -> impl IntoResponse {
    let tweets = (1..=20)
    .into_iter()
    .map(|_| Tweet {
        name: "John Doe".to_string(),
        message: "Hello, world!".to_string(),
        posted_at: "2020-01-01T00:00:00Z".to_string(),
    })
    .collect();
    let home = Home { tweets };
    response::from_template(home)
}

