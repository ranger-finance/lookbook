use dioxus::prelude::*;
// use dioxus_resize_observer::use_size;
// use dioxus_use_mounted::use_mounted;

#[component]
pub fn HorizontalPane(left: Element, right: Element) -> Element {
    let mut width = use_signal(|| 250.);
    let mut is_dragging = use_signal(|| false);

    rsx! {
        div {
            position: "relative",
            flex: 1,
            display: "flex",
            flex_direction: "row",
            onmouseup: move |_| { is_dragging.set(false) },
            prevent_default: if *is_dragging.peek() { "onmousedown onmousemove" } else { "" },
            onmousemove: move |event| {
                if *is_dragging.peek() {
                    width.set(event.data.client_coordinates().x)
                }
            },
            div { display: "flex", flex_direction: "row", width: "{width}px", overflow: "auto",
                {left},
                div {
                    height: "100%",
                    padding: "0 5px",
                    margin: "0 -5px",
                    cursor: "ew-resize",
                    onmousedown: move |_| { is_dragging.set(true) },
                    div { width: "2px", height: "100%", background: "#ccc" }
                }
            }
            {right}
        }
    }
}

#[component]
pub fn VerticalPane(top: Element, bottom: Element) -> Element {
    // let container_ref = use_mounted();
    // let container_size = use_size(container_ref);

    // let mut height = use_signal(|| 0.);

    // let mut count = use_signal(|| 0);

    // use_future(move || {
    //     if *count.peek() <= 2 {
    //         height.set(height / 2.);
    //         count += 1;
    //     }
    //     async {}
    // });

    // let mut is_dragging = use_signal(|| false);

    // is_dragging.peek()
    rsx! {
        div {
            position: "relative",
            flex: 1,
            display: "flex",
            flex_direction: "column",
            // onmounted: move |event| container_ref.onmounted(event),
            div {
                position: "absolute",
                // display: if *is_dragging.peek() { "block" } else { "none" },
                width: "100%",
                height: "100%",
                // onmouseup: move |_| { is_dragging.set(false) },
                // prevent_default: if *is_dragging.peek() { "onmousedown onmousemove" } else { "" },
                // onmousemove: move |event| height.set(container_size.height() - event.data.client_coordinates().y)
            }
            {top},
            div {
                width: "100%",
                padding: "5px 0",
                margin: "-5px 0",
                cursor: "ns-resize",
                // onmousedown: move |_| { is_dragging.set(true) },
                div { height: "2px", width: "100%", background: "#ccc" }
            }
            // div { display: "flex", flex_direction: "column", height: "auto", overflow: "auto", {bottom} }
        }
    }
}
