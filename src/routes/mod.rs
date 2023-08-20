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
    markup_builder, render, AppResult, AppState,
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
                    // Until maud supports more flexible attributes, gotta resort to this.
                    // My specific problem is with trying to set the "x-on:htmx:after-request" attribute,
                    // which contains two colons, which maud doesn't like for some reason.
                    (markup_builder::MarkupBuilder::new("form")
                        .class("post-form")
                        .attribute("hx-post", posts_path)
                        .attribute("hx-target", "#posts")
                        .attribute("hx-select", "#posts li")
                        .attribute("hx-swap", "afterbegin")
                        .attribute("x-init", "$nextTick(() => htmx.process($el))")
                        .attribute("@htmx:after-request", "open = false")
                        .inner_html(html! {
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
                        })
                    )
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
