use axum::{extract::{State, Query}, http::StatusCode, Form};
use axum_extra::routing::TypedPath;
use maud::{html, Markup};
use sea_orm::{entity::*, query::*};
use serde::{Deserialize, Serialize};

use crate::{
    entities::{post, prelude::*},
    render, AppResult, AppState, poster::Poster,
};

#[derive(TypedPath, Deserialize)]
#[typed_path("/replies/:id")]
pub struct RepliesPath {
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct RepliesQuery {
    #[serde(default)]
    pub nested: bool,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/replies/:id/lazy")]
pub struct RepliesLazyPath {
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
    Query(query): Query<RepliesQuery>,
) -> AppResult<Markup> {
    let replies = Post::find()
        .filter(post::Column::ParentPostId.eq(id))
        .order_by_asc(post::Column::Id)
        .all(&state.db)
        .await?;

    // Nested replies are rendered differently
    if query.nested {
        return Ok(html! {
            ul #{"replies-" (id)}
                .replies
                empty:hidden
                ml="3"
                pl="3"
                b="l-2 #038b25"
                flex="~ col self-stretch"
                gap="4"
                role="list"
            {
                @for reply in replies {
                    li flex="~ col" gap="4" {
                        (render::reply(reply))
                    }
                }
            }
        })
    }

    let post = Post::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Not Found: {id}")))?;

    Ok(render::layout(
        "clovers :: replies",
        html! {
            (render::post(post))
            #make-post-container p="2" bg="white" rounded shadow="md" x-data="{ open: false }" {
                button x-on:click="open = true" { "Reply" }
                (render::reply_form_template(id))
            }
            section flex="~ col items-start" gap="4" {
                h2 font="size-5 bold" { "Replies" }
                ul #{"replies-" (id)}
                    ."empty-after-content-['No_replies_yet.']"
                    flex="~ col self-stretch"
                    gap="4"
                    role="list"
                {
                    @for reply in replies {
                        li flex="~ col" gap="4" {
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

    Ok(html! {
        li.fade-in flex="~ col" gap="4" { (render::reply(post)) }
    })
}

pub async fn get_replies_lazy(
    RepliesLazyPath { id }: RepliesLazyPath,
    State(state): State<AppState>,
) -> AppResult<Markup> {
    const LAZY_LIMIT: u64 = 4;

    let replies_path = RepliesPath { id }.with_query_params(RepliesQuery { nested: true });

    let reply_count = Post::find().filter(post::Column::ParentPostId.eq(id)).count(&state.db).await?;

    // Don't load too many replies
    if reply_count > LAZY_LIMIT {
        return Ok(html! {
            ul {
                button
                    hx-get=(replies_path)
                    hx-target="closest ul"
                    hx-swap="outerHTML"
                { "Load " (reply_count) " Replies" }
            }
        })
    }

    Ok(html! {
        div hidden hx-trigger="load"
            hx-get=(replies_path)
            hx-swap="outerHTML"
        { }
    })
}
