/// Auto-generated by sea-orm
mod entities;

mod poster;
mod render;

use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
};
use entities::{post, prelude::*};
use maud::{html, Markup};
use sea_orm::{entity::*, query::*};
use serde::Deserialize;

use crate::poster::Poster;

#[derive(Clone)]
struct AppState {
    db: sea_orm::DatabaseConnection,
}

/// Request body for the `/posts` route.
#[derive(Deserialize)]
struct MakePost {
    content: String,
    poster: String,
}

/// Query parameters for the `/user/:name` route.
#[derive(Deserialize)]
struct GetUser {
    hash: Option<String>,
}

/// Return type for fallible routes.
type AppResult<T> = Result<T, (StatusCode, String)>;

const DATABASE_URL: &str = "sqlite:./database.db?mode=rwc";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use axum::routing::get;
    use migration::MigratorTrait;

    // == DATABASE ==
    let db = sea_orm::Database::connect(DATABASE_URL).await?;
    migration::Migrator::up(&db, None).await?;

    let state = AppState { db };

    // == ROUTES ==
    let post_routes = axum::Router::new()
        .route("/", get(get_posts).post(make_post))
        .route("/new", get(get_post_form))
        .route("/replies/:id", get(get_post_and_replies));

    let app = axum::Router::new()
        .route("/", get(root))
        .route("/user/:name", get(get_user))
        .nest("/posts", post_routes)
        .nest_service("/static", tower_http::services::ServeDir::new("static"))
        .with_state(state);

    // == RUN ==
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root(State(state): State<AppState>) -> AppResult<Markup> {
    let posts = Post::find()
        .filter(post::Column::ParentPostId.is_null())
        .order_by_desc(post::Column::Id)
        .limit(3)
        .all(&state.db)
        .await
        .map_err(db_err_to_response)?;

    Ok(render::layout(
        "clovers",
        html! {
            #make-post-container {
                button hx-get="/posts/new" hx-target="#post-form" { "Make a Post" }
                #post-form { }
            }
            figure {
                figcaption { "Recent Posts" }
                ul #posts {
                    @for post in &posts {
                        li { (render::post(post)) }
                    }
                }
                a href="/posts" { "View More" }
            }
        },
    ))
}

async fn get_user(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Query(query): Query<GetUser>,
) -> AppResult<Markup> {
    use base64ct::Encoding;

    let bytes = query
        .hash
        .as_deref()
        .map(base64ct::Base64UrlUnpadded::decode_vec)
        .transpose()
        .map_err(|_| (StatusCode::BAD_REQUEST, String::from("Invalid Hash")))?;

    let posts = Post::find()
        .filter(post::Column::Name.eq(&name))
        .apply_if(bytes, |query, bytes| {
            query.filter(post::Column::Hash.eq(bytes))
        })
        .order_by_desc(post::Column::Id)
        .all(&state.db)
        .await
        .map_err(db_err_to_response)?;

    let rendered_posts = html! {
        @for post in &posts {
            li { (render::post(post)) }
        }
    };

    Ok(render::layout(
        "clovers :: posts",
        html! {
            span {
                "Searching for posts by "
                (render::poster(&name, query.hash.as_deref()))
            }
            ul #posts {
                (rendered_posts)
            }
        },
    ))
}

async fn get_post_form() -> Markup {
    html! {
        form.post-form
            hx-disinherit="*"
            hx-post="/posts"
            hx-swap="delete"
            hx-select-oob="#posts:afterbegin"
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
            // TODO: use alpine or something to eliminate ajax request
            a href="#"
                hx-get="/"
                hx-target="closest .post-form"
                hx-swap="delete"
            {
                "Cancel"
            }
        }
    }
}

async fn get_posts(State(state): State<AppState>) -> AppResult<Markup> {
    let posts = Post::find()
        .filter(post::Column::ParentPostId.is_null())
        .order_by_desc(post::Column::Id)
        .all(&state.db)
        .await
        .map_err(db_err_to_response)?;

    let rendered_posts = html! {
        @for post in &posts {
            li { (render::post(post)) }
        }
    };

    Ok(render::layout(
        "clovers :: posts",
        html! {
            ul #posts {
                (rendered_posts)
            }
        },
    ))
}

async fn make_post(State(state): State<AppState>, Form(post): Form<MakePost>) -> AppResult<Markup> {
    if post.content.is_empty() {
        return Ok(Markup::default());
    }

    let Poster { name, hash } = post.poster.parse().expect("Infallible");

    let post = post::ActiveModel {
        content: ActiveValue::Set(post.content),
        name: ActiveValue::Set(name),
        hash: ActiveValue::Set(hash),
        ..Default::default()
    };

    let post = Post::insert(post)
        .exec_with_returning(&state.db)
        .await
        .map_err(db_err_to_response)?;

    let rendered_post = render::post(&post);

    Ok(html! {
        ul #posts {
            li.new-post { (rendered_post) }
        }
    })
}

async fn get_post_and_replies(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Markup> {
    let post = Post::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(db_err_to_response)?
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Not Found: {id}")))?;

    // TODO: show replies

    Ok(render::layout(
        "clovers :: post",
        html! {
            (render::post(&post))
        },
    ))
}

fn db_err_to_response(_err: sea_orm::error::DbErr) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        String::from("Database Error"),
    )
}
