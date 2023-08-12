use maud::{html, Markup};

pub fn layout(title: &str, body: Markup) -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            head {
                title { (title) }
                script src="https://unpkg.com/htmx.org@1.9.4" { }
                link rel="stylesheet" href="https://unpkg.com/modern-normalize";
                style { (include_str!("style.css")) }
            }
            body hx-boost="true" {
                (body)
            }
        }
    }
}

pub fn post_button_item() -> Markup {
    html! {
        li { button hx-get="/posts/new" hx-swap="outerHTML" { "Make a Post" } }
    }
}

pub fn post_item(content: &str) -> Markup {
    html! {
        li.post { (content) }
    }
}
