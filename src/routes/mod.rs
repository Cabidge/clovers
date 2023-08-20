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
            #make-post-container x-data="{ open: false }" {
                button x-on:click="open = true" { "Make a Post" }
                template x-if="open" {
                    form.post-form
                        hx-post=(posts_path)
                        hx-target="#posts"
                        hx-select="#posts li"
                        hx-swap="afterbegin"
                        x-init="$nextTick(() => htmx.process($el))"
                        // This has to be done next tick because otherwise htmx won't execute the post request.
                        // An alternative could be to use x-show and clear the form, but that would be more complicated.
                        x-on:submit="$nextTick(() => open = false)"
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
                        a href="#" x-on:click="open = false" { "Cancel" }
                    }
                }
            }
            figure {
                figcaption { "Recent Posts" }
                ul #posts role="list" {
                    @for post in posts {
                        li { (render::post(post)) }
                    }
                }
                a href=(posts_path) { "View More" }
            }
        },
    ))
}
