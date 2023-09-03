use std::fmt::Display;

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
                style {"
                    [un-cloak] { display: none; }

                    .fade-in {
                        transform: translateY(0);
                        opacity: 1;
                        transition: all 0.3s ease-out;
                    }

                    .fade-in.htmx-added {
                        transform: translateY(-2rem);
                        opacity: 0;
                    }
                "}
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@unocss/reset/tailwind.min.css";
                //link rel="stylesheet" href="https://unpkg.com/modern-normalize";
                //link rel="stylesheet" href="/static/style.css";
            }
            body bg="#f0f0f0" hx-boost="true" un-cloak {
                header bg="white" z="10" sticky top="0" p="8" shadow="md" {
                    h1 font="size-8 bold" { (link("/", "clovers")) }
                }
                main mx="a" p="x-8 y-12" max-w="4xl" flex="~ col" gap="8" {
                    (body)
                }
            }
        }
    }
}

pub fn link(href: impl Display, text: impl maud::Render) -> Markup {
    html! {
        a text="#038b25" hover:underline href=(href) { (text) }
    }
}

pub fn relative_time(time: chrono::DateTime<chrono::Utc>) -> Markup {
    html! {
        time datetime=(time) title=(time) {
            (crate::relative_time::Relative(time))
        }
    }
}

pub fn post_form_body() -> &'static Markup {
    use std::sync::OnceLock;

    static CELL: OnceLock<Markup> = OnceLock::new();

    CELL.get_or_init(|| html! {
        label flex="~ col" {
            span { "Name (optional)" }
            input name="poster" placeholder="Anonymous" autocomplete="off";
        }
        label flex="~ col" {
            span { "Content" }
            textarea resize="none" rows="10" name="content" placeholder="What's on your mind?" { }
        }
        div flex="~ row justify-end" gap="4" {
            button hover:underline rounded type="button" x-on:click="open = false" { "Cancel" }
            button p="x-4 y-1"
                rounded
                bg="#038b25"
                text="white"
                scale="100 hover:110 active:90"
                transition="transform-100"
                ease-in
            { "Post" }
        }
    })
}

pub fn post_list(children: Markup) -> Markup {
    html! {
        ul #posts w="full" flex="~ col" gap="4" role="list" {
            (children)
        }
    }
}

pub fn posts(posts: Vec<post::Model>) -> Markup {
    post_list(
        html! {
            @for post in posts {
                li { (self::post(post)) }
            }
        }
    )
}

pub fn post(post: post::Model) -> Markup {
    use crate::routes::replies::RepliesPath;

    let replies_path = RepliesPath { id: post.id };

    html! {
        article p="8" bg="white" shadow="md" flex="~ col" gap="4" {
            span { "Posted " (relative_time(post.created_at)) }
            (poster_link(post.name, post.hash.as_deref()))
            pre font-sans { (post.content) }
            (link(replies_path, "View Replies"))
        }
    }
}

pub fn reply(post: post::Model) -> Markup {
    use crate::routes::replies::{RepliesPath, RepliesLazyPath};

    let id = post.id;
    let replies_path = RepliesPath { id };
    let replies_lazy_path = RepliesLazyPath { id };

    html! {
        article p="4" bg="white" rounded shadow="md" flex="~ col" gap="4" {
            header {
                (poster_link(post.name, post.hash.as_deref()))
                span { " Posted " (link(replies_path, relative_time(post.created_at))) }
            }
            pre font-sans { (post.content) }
            footer x-data="{ open: false }" {
                button x-show="!open" x-on:click="open = true" { "Reply" }
                (reply_form_template(id))
            }
        }
        div hidden hx-trigger="revealed" hx-get=(replies_lazy_path) hx-swap="outerHTML" { }
    }
}

pub fn reply_form_template(post_id: i32) -> Markup {
    use crate::routes::replies::RepliesPath;

    let replies_path = RepliesPath { id: post_id };

    html! {
        template x-if="open" {
            form
                flex="~ col"
                gap="4"
                hx-post=(replies_path)
                hx-target={"#replies-" (post_id)}
                hx-swap="afterbegin"
                x-init="$nextTick(() => htmx.process($el))"
                x-on:submit="$nextTick(() => open = false)"
            { (post_form_body()) }
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
        a href=(user_path) {
            (rendered_poster)
        }
    }
}

pub fn poster(name: &str, hash: Option<&str>) -> Markup {
    html! {
        span.poster font-bold {
            span {
                (name)
            }
            @if let Some(tripcode) = hash {
                " (" span inline-block align-btm max-w="20 hover:none" truncate text="0.9rem #038b25" font-mono { "#" (tripcode) } ")"
            }
        }
    }
}
