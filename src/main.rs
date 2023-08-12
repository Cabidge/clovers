mod render;

use std::sync::{Arc, Mutex};

use axum::{extract::State, Form};
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
            button { "Post" }
        }
    }
}

async fn make_post(State(state): State<AppState>, Form(post): Form<MakePost>) -> Markup {
    state.posts.lock().unwrap().push(post.content.clone());

    html! {
        (render::post_button_item())
        (render::post_item(&post.content))
    }
}
