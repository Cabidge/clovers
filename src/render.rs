use axum_extra::routing::TypedPath;
use maud::{html, Markup};

use crate::entities::post;

pub fn layout(title: &str, body: Markup) -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            head {
                title { (title) }
                script src="https://unpkg.com/htmx.org@1.9.4" { }
                script src="https://unpkg.com/alpinejs" defer { }
                link rel="stylesheet" href="https://unpkg.com/modern-normalize";
                link rel="stylesheet" href="/static/style.css";
            }
            body hx-boost="true" {
                header {
                    h1 { a href="/" { "clovers" } }
                }
                main {
                    (body)
                }
            }
        }
    }
}

pub fn post(post: post::Model) -> Markup {
    use crate::routes::replies::RepliesPath;

    let replies_path = RepliesPath { id: post.id };

    html! {
        article.post {
            span { "Posted " (post.created_at) }
            (poster_link(post.name, post.hash.as_deref()))
            pre.post-content { (post.content) }
            a href=(replies_path) { "View Replies" }
        }
    }
}

pub fn poster_link(name: String, bytes: Option<&[u8]>) -> Markup {
    use crate::routes::user::{UserPath, UserQuery};
    use base64ct::Encoding;

    let serialized_hash = bytes.map(base64ct::Base64UrlUnpadded::encode_string);

    let rendered_poster = poster(&name, serialized_hash.as_deref());

    let user_path = UserPath { name }.with_query_params(UserQuery {
        hash: serialized_hash,
    });

    html! {
        a.poster-link href=(user_path) {
            (rendered_poster)
        }
    }
}

pub fn poster(name: &str, hash: Option<&str>) -> Markup {
    html! {
        span.poster {
            span.poster-name {
                (name)
            }
            @if let Some(tripcode) = hash {
                " (" span.tripcode { "#" (tripcode) } ")"
            }
        }
    }
}
