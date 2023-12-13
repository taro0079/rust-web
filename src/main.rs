use askama::Template;
use axum::{response::{Html, IntoResponse}, routing::get, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust-web=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    let tweets = (1..=20)
    .into_iter()
    .map(|_| TweetView {
        name: "John Doe".to_string(),
        message: "Hello, world!".to_string(),
        posted_at: "2020-01-01T00:00:00Z".to_string(),
    })
    .collect();
    let home = Home { tweets };
    Html(home.render().unwrap()).into_response()
}

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    tweets: Vec<TweetView>,
}

struct TweetView {
    name: String,
    message: String,
    posted_at: String,
}
