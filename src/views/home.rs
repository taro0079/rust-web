use askama::Template;

use crate::views::partial::tweet::Tweet;

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home {
    pub tweets: Vec<Tweet>,
}
