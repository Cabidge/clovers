mod render;

use std::sync::{Arc, Mutex};

use axum::{extract::State, response::Redirect, Form};
use maud::{html, Markup};
use serde::Deserialize;

#[derive(Clone)]
struct AppState {
    posts: Arc<Mutex<Vec<String>>>,
}

#[derive(Deserialize)]
struct MakePost {
    content: String,
}

#[tokio::main]
async fn main() {
    use axum::routing::{get, post};

    let state = AppState {
        posts: Arc::new(Mutex::new(vec![
            "Hello".into(),
            "World".into(),
            "Foo".into(),
        ])),
    };

    let app = axum::Router::new()
        .route("/", get(root))
        .route("/post/new", get(make_post_form))
        .route("/post", post(make_post))
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
            a href="/post/new" { "Make a Post" }
            ul #posts {
                @for post in posts.iter().rev() {
                    li.post { (post) }
                }
            }
        },
    )
}

async fn make_post_form() -> Markup {
    render::layout(
        "clovers | Make a Post",
        html! {
            form method="post" action="/post" {
                textarea rows="10" cols="80" name="content" { }
                button { "Post" }
            }
        },
    )
}

async fn make_post(State(state): State<AppState>, Form(post): Form<MakePost>) -> Redirect {
    state.posts.lock().unwrap().push(post.content.clone());
    Redirect::to("/")
}
