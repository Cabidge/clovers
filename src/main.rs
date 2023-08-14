mod post;
mod poster;
mod render;

use std::sync::{Arc, Mutex};

use axum::{extract::State, Form};
use maud::{html, Markup};
use serde::Deserialize;

use crate::{post::Post, poster::Poster};

#[derive(Clone)]
struct AppState {
    posts: Arc<Mutex<Vec<post::Post>>>,
}

#[derive(Deserialize)]
struct MakePost {
    content: String,
    poster: String,
}

#[tokio::main]
async fn main() {
    use axum::routing::{get, post};

    let state = AppState {
        posts: Arc::new(Mutex::new(vec![
            Post {
                content: "Hello, world!".into(),
                poster: Poster::new(),
            },
            Post {
                content: "foo...".into(),
                poster: Poster::with_name("bar"),
            },
            Post {
                content: "I am secretive...".into(),
                poster: Poster::with_name_and_secret("baz", "hunter2"),
            },
        ])),
    };

    let post_routes = axum::Router::new()
        .route("/", post(make_post))
        .route("/new", get(get_post_form));

    let app = axum::Router::new()
        .route("/", get(root))
        .nest("/posts", post_routes)
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(State(state): State<AppState>) -> Markup {
    let posts = state.posts.lock().unwrap();

    render::layout(
        "clovers",
        html! {
            ul #posts {
                (render::post_button_item())
                @for post in posts.iter().rev() {
                    (render::post_item(post))
                }
            }
        },
    )
}

async fn get_post_form() -> Markup {
    html! {
        form hx-post="/posts" hx-target="closest li" hx-swap="outerHTML" {
            textarea rows="10" cols="80" name="content" { }
            input name="poster" placeholder="Name (optional)" { }
            button { "Post" }
        }
    }
}

async fn make_post(State(state): State<AppState>, Form(post): Form<MakePost>) -> Markup {
    let post = Post {
        content: post.content,
        poster: post.poster.parse().expect("Infallible"),
    };

    let post_item = render::post_item(&post);

    state.posts.lock().unwrap().push(post);

    html! {
        (render::post_button_item())
        (post_item)
    }
}
