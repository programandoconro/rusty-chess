use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent};
use yew::prelude::*;
use yew::{html, Callback};

struct Coordinates {
    x: f64,
    y: f64,
}

#[function_component]
pub fn Pawn() -> Html {
    let coordinates: UseStateHandle<Coordinates> = use_state(|| Coordinates { x: 0.0, y: 0.0 });
    let is_dragging: UseStateHandle<bool> = use_state(|| false);

    let onmousedown: Callback<MouseEvent> = {
        let is_dragging: UseStateHandle<bool> = is_dragging.clone();
        Callback::from(move |event: MouseEvent| -> () {
            event.prevent_default();
            is_dragging.set(true);
        })
    };
    let onmouseup: Callback<MouseEvent> = {
        let is_dragging: UseStateHandle<bool> = is_dragging.clone();
        Callback::from(move |event: MouseEvent| -> () {
            event.prevent_default();
            is_dragging.set(false);
        })
    };
    let onmousemove: Callback<MouseEvent> = {
        let coordinates: UseStateHandle<Coordinates> = coordinates.clone();

        Callback::from(move |event: MouseEvent| -> () {
            event.prevent_default();
            let rect = event
                .target()
                .expect("mouse event doesn't have a target")
                .dyn_into::<HtmlElement>()
                .expect("event target should be of type HtmlElement")
                .get_bounding_client_rect();
            let x = (event.client_x() as f64) - rect.left() - 30.0;
            let y = (event.client_y() as f64) - rect.top() - 30.0;
            web_sys::console::log_1(&x.to_string().into());
            if *is_dragging == true {
                coordinates.set(Coordinates { x, y });
            }
        })
    };

    html! {
        <div
        class="pawn"
        onmouseleave={onmouseup.clone()}
        onmousedown={onmousedown}
        onmouseup={onmouseup}
        onmousemove={onmousemove}
        style={format!("left: {}px; top: {}px;",
         coordinates.x, coordinates.y)}>
         <img src="img/pawn.png" width="50px" height="50px" />
        </div>

    }
}
