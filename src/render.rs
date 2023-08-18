use maud::{html, Markup};

use crate::entities::post;

pub fn layout(title: &str, body: Markup) -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            head {
                title { (title) }
                script src="https://unpkg.com/htmx.org@1.9.4" { }
                link rel="stylesheet" href="https://unpkg.com/modern-normalize";
                link rel="stylesheet" href="/static/style.css";
            }
            body hx-boost="true" {
                (body)
            }
        }
    }
}

pub fn post(post: &post::Model) -> Markup {
    html! {
        article.post {
            (poster_link(&post.name, post.hash.as_deref()))
            pre.post-content { (post.content) }
            a href={"/posts/replies/" (post.id)} { "View Replies" }
        }
    }
}

pub fn poster_link(name: &str, bytes: Option<&[u8]>) -> Markup {
    use base64ct::Encoding;

    let serialized_hash = bytes.map(base64ct::Base64UrlUnpadded::encode_string);

    html! {
        a.poster-link href={
            "/user/" (name)
            @if let Some(hash) = &serialized_hash {
                "?hash=" (hash)
            }
        } {
            (poster(name, serialized_hash.as_deref()))
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
