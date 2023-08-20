use axum::{extract::State, Form};
use axum_extra::routing::TypedPath;
use maud::{html, Markup};
use sea_orm::{entity::*, query::*};
use serde::Deserialize;

use crate::{
    entities::{post, prelude::*},
    poster::Poster,
    render, AppResult, AppState,
};

#[derive(TypedPath, Deserialize)]
#[typed_path("/posts")]
pub struct PostsPath;

/// Request body for the `/posts` route.
#[derive(Deserialize)]
pub struct MakePost {
    content: String,
    poster: String,
}

pub async fn get_posts(_: PostsPath, State(state): State<AppState>) -> AppResult<Markup> {
    let posts = Post::find()
        .filter(post::Column::ParentPostId.is_null())
        .order_by_desc(post::Column::Id)
        .all(&state.db)
        .await?;

    let rendered_posts = html! {
        @for post in posts {
            li { (render::post(post)) }
        }
    };

    Ok(render::layout(
        "clovers :: posts",
        html! {
            ul #posts role="list" {
                (rendered_posts)
            }
        },
    ))
}

pub async fn make_post(
    _: PostsPath,
    State(state): State<AppState>,
    Form(post): Form<MakePost>,
) -> AppResult<Markup> {
    if post.content.is_empty() {
        return Ok(Markup::default());
    }

    let Poster { name, hash } = post.poster.parse().expect("Infallible");

    let post = post::ActiveModel {
        content: ActiveValue::Set(post.content),
        name: ActiveValue::Set(name),
        hash: ActiveValue::Set(hash),
        created_at: ActiveValue::Set(chrono::Utc::now()),
        ..Default::default()
    };

    let post = Post::insert(post).exec_with_returning(&state.db).await?;

    let rendered_post = render::post(post);

    Ok(html! {
        ul #posts role="list" {
            li.new-post { (rendered_post) }
        }
    })
}
