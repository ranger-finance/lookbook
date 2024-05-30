use dioxus::prelude::*;
// use dioxus_material::use_theme;
use serde::{Deserialize, Serialize};

/// A controllable property.
pub trait Control: Sized {
    type State;

    /// Create the initial state.
    fn state(default: Option<impl Into<Self>>) -> Self::State;

    /// Convert the current state to `Self`.
    fn from_state(state: Signal<&Self::State>) -> Self;

    /// Render the controller element.
    fn control(name: &'static str, state: Signal<Self::State>) -> Element;
}

impl Control for &str {
    type State = String;

    fn state(default: Option<impl Into<Self>>) -> Self::State {
        default
            .map(Into::into)
            .map(String::from)
            .unwrap_or_default()
    }

    fn from_state(state: Signal<&Self::State>) -> Self {
        let value_ref = &state.read();
        value_ref.as_str()
    }

    fn control(_name: &'static str, mut state: Signal<Self::State>) -> Element {
        let value = "{state.read()}";

        rsx! {Input {
            value: value,
            oninput: move |event: FormEvent| {
                let val = event.data.value().clone();
               *state.write() = val;
            }
        }}
    }
}

#[derive(Default)]
pub struct Json<T>(pub T);

impl<T> From<T> for Json<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

// impl<T> IntoDynNode for Json<T>
// where
//     T: Clone + Default + Deserialize + Serialize,
// {
//     fn into_vnode(self, cx: &ScopeState) -> dioxus::core::DynamicNode<'a> {
//         let s = serde_json::to_string(&self.0).unwrap();
//         cx.make_node(s)
//     }

//     fn into_dyn_node(self, cx: &'a ScopeState) -> dioxus::core::DynamicNode<'a> {
//         let s = serde_json::to_string(&self.0).unwrap();
//         cx.make_node(s)
//     }
// }

impl<T> Control for Json<T>
where
    T: 'static + Clone + Default + Deserialize<'static> + Serialize,
{
    type State = T;

    fn state(default: Option<impl Into<Self>>) -> Self::State {
        default.map(Into::into).unwrap_or_default().0
    }

    fn from_state(state: Signal<&Self::State>) -> Self {
        Self(state.read().clone())
    }

    fn control(_name: &'static str, mut state: Signal<Self::State>) -> Element {
        // let value = state.read();
        // let value = &*value;
        // let json = ;
        // let value = json.as_str();
        let value = "{serde_json::to_string(&*state.read()).unwrap().as_str()}";
        // let json = serde_json::to_string(&*state.read()).unwrap();
        // let value = json.as_str();
        // let thing = "stuff";

        rsx! {Input {
            // value: "{json}",
            value: value,
            oninput: move |event: FormEvent| {
                let value = "{event.data.value().clone().as_str()}";
                // let value = clone.as_str();
                if let Ok(new_state) = serde_json::from_str::<T>(value) {
                    *state.write() = new_state;
                }
            }
        }}
    }
}

#[component]
fn Input(value: &'static str, oninput: EventHandler<FormEvent>) -> Element {
    // let theme = use_theme(cx);

    rsx! {input {
        border: "2px solid #e7e7e7",
        padding: "10px",
        // border_radius: &*theme.border_radius_small,
        // font_size: "{theme.label_small}px",
        outline: "none",
        background: "none",
        value: "{value}",
        oninput: move |event| oninput.call(event)
    }}
}
