use maud::{html, Markup};

#[tokio::main]
async fn main() {
    use axum::routing::get;

    let app = axum::Router::new().route("/", get(root));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Markup {
    render_base(
        "Hello, World",
        html! {
            h1 { "Hello, World!" }
        },
    )
}

fn render_base(title: &str, body: Markup) -> Markup {
    html! {
        html {
            head {
                title { (title) }
            }
            body {
                (body)
            }
        }
    }
}
