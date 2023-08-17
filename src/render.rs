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

pub fn post(post: &post::Model) -> Markup {
    use base64ct::Encoding;

    let serialized_hash = post
        .hash
        .as_deref()
        .map(base64ct::Base64UrlUnpadded::encode_string);

    let poster_query = {
        let mut queries = vec![("name", post.name.as_str())];
        if let Some(hash) = &serialized_hash {
            queries.push(("hash", hash.as_str()));
        }

        querystring::stringify(queries)
    };

    html! {
        article.post {
            a.poster-link href={"/posts?" (poster_query)} {
                (poster(&post.name, serialized_hash.as_deref()))
            }
            pre.post-content { (post.content) }
            a href={"/posts/replies/" (post.id)} { "View Replies" }
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
