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
                script src="https://cdn.jsdelivr.net/npm/@unocss/runtime/attributify.global.js" { }
                style {" [un-cloak] { display: none; } "}
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@unocss/reset/tailwind.min.css";
                //link rel="stylesheet" href="https://unpkg.com/modern-normalize";
                //link rel="stylesheet" href="/static/style.css";
            }
            body ."bg-#f0f0f0" hx-boost="true" un-cloak {
                header class="bg-white z-10 sticky top-0 px-20 py-8 shadow" {
                    h1 font="size-8 bold" { a class="text-#038b25 hover:underline" href="/" { "clovers" } }
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

pub fn reply(post: post::Model) -> Markup {
    use crate::routes::replies::{RepliesPath, RepliesLazyPath};

    let id = post.id;
    let replies_path = RepliesPath { id };
    let replies_lazy_path = RepliesLazyPath { id };

    html! {
        article.reply {
            header {
                (poster_link(post.name, post.hash.as_deref()))
                span { " Posted " a href=(replies_path) { (post.created_at) } }
            }
            pre.post-content { (post.content) }
            footer x-data="{ open: false }" {
                button x-on:click="open = true" { "Reply" }
                (reply_form_template(id))
            }
        }
        div.hidden hx-trigger="revealed" hx-get=(replies_lazy_path) hx-swap="outerHTML" { }
    }
}

pub fn reply_form_template(post_id: i32) -> Markup {
    use crate::routes::replies::RepliesPath;

    let replies_path = RepliesPath { id: post_id };

    html! {
        template x-if="open" {
            form.post-form
                hx-post=(replies_path)
                hx-target={"#replies-" (post_id)}
                hx-swap="afterbegin"
                x-init="$nextTick(() => htmx.process($el))"
                x-on:submit="$nextTick(() => open = false)"
            {
                label {
                    span { "Name (optional)" }
                    input name="poster" placeholder="Anonymous" autocomplete="off";
                }
                label {
                    span { "Content" }
                    textarea rows="10" name="content" placeholder="What's on your mind?" { }
                }
                button { "Post" }
                a href="#" x-on:click="open = false" { "Cancel" }
            }
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
