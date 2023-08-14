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
    html! {
        article.post {
            span.poster {
                (post.poster.name)
                @if let Some(hash) = post.poster.hash() {
                    i { " #" (hash) }
                }
            }
            pre.post-content { (post.content) }
        }
    }
}
