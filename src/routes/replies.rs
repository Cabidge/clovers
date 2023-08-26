use axum::{extract::State, http::StatusCode, Form};
use axum_extra::routing::TypedPath;
use maud::{html, Markup};
use sea_orm::{entity::*, query::*};
use serde::Deserialize;

use crate::{
    entities::{post, prelude::*},
    render, AppResult, AppState, poster::Poster,
};

#[derive(TypedPath, Deserialize)]
#[typed_path("/replies/:id")]
pub struct RepliesPath {
    pub id: i32,
}

/// Request body for the `/replies/:id` route.
#[derive(Deserialize)]
pub struct MakeReply {
    content: String,
    poster: String,
}

pub async fn get_replies(
    RepliesPath { id }: RepliesPath,
    State(state): State<AppState>,
) -> AppResult<Markup> {
    let post = Post::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Not Found: {id}")))?;

    let replies = Post::find()
        .filter(post::Column::ParentPostId.eq(id))
        .order_by_asc(post::Column::Id)
        .all(&state.db)
        .await?;

    let post_id = post.id;

    Ok(render::layout(
        "clovers :: replies",
        html! {
            (render::post(post))
            #make-post-container x-data="{ open: false }" {
                button x-on:click="open = true" { "Reply" }
                (render::reply_form_template(post_id))
            }
            figure {
                figcaption { "Replies" }
                ul #{"replies-" (post_id)} .replies role="list" {
                    @for reply in replies {
                        li {
                            (render::reply(reply))
                        }
                    }
                }
            }
        },
    ))
}

pub async fn make_reply(
    RepliesPath { id }: RepliesPath,
    State(state): State<AppState>,
    Form(post): Form<MakeReply>,
) -> AppResult<Markup> {
    if post.content.is_empty() {
        return Ok(Markup::default());
    }

    let Poster { name, hash } = post.poster.parse().expect("Infallible");

    let post = post::ActiveModel {
        content: ActiveValue::Set(post.content),
        name: ActiveValue::Set(name),
        hash: ActiveValue::Set(hash),
        parent_post_id: ActiveValue::Set(Some(id)),
        ..Default::default()
    };

    let post = Post::insert(post).exec_with_returning(&state.db).await?;

    let rendered_reply = render::reply(post);

    Ok(rendered_reply)
}