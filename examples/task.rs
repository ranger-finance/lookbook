use dioxus::prelude::*;
use lookbook::{Json, LookBook};
use lookbook_macros::preview;
use tracing::Level;

#[component]
pub fn MyComponent() -> Element {
    rsx! {
        "hello my component"
    }
}

///
///     /// Label of the task.
// #[lookbook(default = "Ice skating")]
// label: &str,

/// Content of the task.
// #[lookbook(default = "Central Park")]
// content: &str,

// /// List of tags.
// #[lookbook(default = vec![String::from("A")])]
// tags: Json<Vec<String>>,

/// To-Do Task.
#[preview]
pub fn TaskPreview() -> Element {
    rsx! {
        div {
            // h4 { "{label}" }
            // p { "{content}" }
            // div { tags.0.iter().map(|tag| rsx!{li { "{tag}" }}) }
            MyComponent {}
        }
    }
}

// #[preview]
// pub fn TaskPreview<'a>(
//     cx: Scope<'a>,

//     /// Label of the task.
//     #[lookbook(default = "Ice skating")]
//     label: &'a str,

//     /// Content of the task.
//     #[lookbook(default = "Central Park")]
//     content: &'a str,

//     /// List of tags.
//     #[lookbook(default = vec![String::from("A")])]
//     tags: Json<Vec<String>>,
// ) -> Element<'a> {
//     rsx! {
//         div {
//             h4 { "{label}" }
//             p { "{content}" }
//             // div { tags.0.iter().map(|tag| render!(li { "{tag}" })) }
//         }
//     }
// }

fn app() -> Element {
    rsx! {
        LookBook {
            home: |()| rsx! {"Home"},
            previews: [TaskPreview]
        }
    }
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(app)
}
