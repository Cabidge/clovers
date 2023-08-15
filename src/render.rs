use maud::{html, Markup};

use crate::post::Post;

pub fn layout(title: &str, body: Markup) -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            head {
                title { (title) }
                script src="https://unpkg.com/htmx.org@1.9.4" { }
                link rel="stylesheet" href="https://unpkg.com/modern-normalize";
                style { (include_str!("style.css")) }
            }
            body hx-boost="true" {
                (body)
            }
        }
    }
}

pub fn post_button() -> Markup {
    html! {
        button hx-get="/posts/new" hx-swap="outerHTML" { "Make a Post" }
    }
}

pub fn post(post: &Post) -> Markup {
    let poster_hash = post.poster.hash();

    let mut queries = vec![("name", post.poster.name.as_str())];
    if let Some(hash) = &poster_hash {
        queries.push(("hash", hash.as_str()));
    }

    let poster_query = querystring::stringify(queries);

    html! {
        article.post {
            a.poster href={"/posts?" (poster_query)} {
                (post.poster.name)
                @if let Some(hash) = poster_hash {
                    span.tripcode { " #" (hash) }
                }
            }
            pre.post-content { (post.content) }
        }
    }
}
