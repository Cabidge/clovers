pub mod posts;
pub mod replies;
pub mod user;

use axum::extract::State;
use axum_extra::routing::TypedPath;
use maud::{html, Markup};
use sea_orm::{entity::*, query::*};
use serde::Deserialize;

use crate::{
    entities::{post, prelude::*},
    render, AppResult, AppState,
};

#[derive(TypedPath, Deserialize)]
#[typed_path("/")]
pub struct RootPath;

pub async fn root(_: RootPath, State(state): State<AppState>) -> AppResult<Markup> {
    let posts = Post::find()
        .filter(post::Column::ParentPostId.is_null())
        .order_by_desc(post::Column::Id)
        .limit(3)
        .all(&state.db)
        .await?;

    let posts_path = posts::PostsPath::PATH;

    Ok(render::layout(
        "clovers",
        html! {
            section p="8" bg="white" rounded shadow="md" x-data="{ open: false }" {
                button x-on:click="open = true" x-show="!open" { "Make a Post" }
                template x-if="open" {
                    form
                        flex="~ col"
                        gap="4"
                        hx-post=(posts_path)
                        hx-target="#posts"
                        hx-select="#posts li"
                        hx-swap="afterbegin"
                        x-init="$nextTick(() => htmx.process($el))"
                        // This has to be done next tick because otherwise htmx won't execute the post request.
                        // An alternative could be to use x-show and clear the form, but that would be more complicated.
                        x-on:submit="$nextTick(() => open = false)"
                    {
                        label flex="~ col" {
                            span { "Name (optional)" }
                            input name="poster" placeholder="Anonymous" autocomplete="off";
                        }
                        label flex="~ col" {
                            span { "Content" }
                            textarea resize="none" rows="10" name="content" placeholder="What's on your mind?" { }
                        }
                        div flex="~ row justify-end" gap="4" {
                            button p="x-2 y-1" b="~ black" rounded { "Post" }
                            button p="x-2 y-1" b="~ black" rounded type="button" x-on:click="open = false" { "Cancel" }
                        }
                    }
                }
            }
            section flex="~ col items-start" gap="4" {
                h2 font="size-5 bold" { "Recent Posts" }
                (render::posts(posts))
                (render::link(posts_path, "View More"))
            }
        },
    ))
}
