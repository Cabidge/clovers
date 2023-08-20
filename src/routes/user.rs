use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use axum_extra::routing::TypedPath;
use maud::{html, Markup};
use sea_orm::{entity::*, query::*};
use serde::{Deserialize, Serialize};

use crate::{
    entities::{post, prelude::*},
    render, AppResult, AppState,
};

#[derive(TypedPath, Deserialize)]
#[typed_path("/user/:name")]
pub struct UserPath {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserQuery {
    pub hash: Option<String>,
}

pub async fn search_user(
    UserPath { name }: UserPath,
    State(state): State<AppState>,
    Query(query): Query<UserQuery>,
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
        .await?;

    let rendered_posts = html! {
        @for post in posts {
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
            ul #posts role="list" {
                (rendered_posts)
            }
        },
    ))
}
