use yew::prelude::*;
use yew::{html, Callback};

struct Coordinates {
    x: Option<i32>,
    y: Option<i32>,
}

#[function_component]
pub fn Pawn() -> Html {
    let coordinates: UseStateHandle<Coordinates> = use_state(|| Coordinates { x: None, y: None });
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
            let x = event.client_x() - 30;
            let y = event.client_y() - 30;
            //web_sys::console::log_1(&x.to_string().into());
            if *is_dragging == true {
                coordinates.set(Coordinates {
                    x: Some(x),
                    y: Some(y),
                });
            }
        })
    };

    let set_position = || -> Option<String> {
        match coordinates.x.is_some() && coordinates.y.is_some() {
            true => Some(format!(
                "left: {}px; top: {}px;",
                coordinates.x.unwrap(),
                coordinates.y.unwrap()
            )),
            false => None,
        }
    };

    html! {
        
        <img
        class="pawn"
        onmouseleave={onmouseup.clone()}
        onmousedown={onmousedown}
        onmouseup={onmouseup}
        onmousemove={onmousemove}
        style={set_position()}
        src="img/pawn.png" width="50px" height="50px" />

    }
}
