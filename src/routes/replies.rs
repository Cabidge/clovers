use axum::{extract::State, http::StatusCode};
use axum_extra::routing::TypedPath;
use maud::{html, Markup};
use sea_orm::{entity::*, query::*};
use serde::Deserialize;

use crate::{
    entities::{post, prelude::*},
    render, AppResult, AppState,
};

#[derive(TypedPath, Deserialize)]
#[typed_path("/replies/:id")]
pub struct RepliesPath {
    pub id: i32,
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
