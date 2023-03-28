use yew::prelude::*;
use yew::{html, Callback};

struct Coordinates {
    x: i32,
    y: i32,
}

#[function_component]
pub fn Pawn() -> Html {
    let coordinates: UseStateHandle<Coordinates> = use_state(|| Coordinates { x: 0, y: 0 });
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
            let y = event.client_y()- 30;
            //web_sys::console::log_1(&x.to_string().into());
            if *is_dragging == true {
                coordinates.set(Coordinates { x, y });
            }
        })
    };

    let set_position = || -> String {
        let coordinates = coordinates.clone();

        if coordinates.x != 0 && coordinates.y != 0 {
            format!("left: {}px; top: {}px;", coordinates.x, coordinates.y)
        } else {
            "".to_string()
        }
    };

    html! {
        <div
        class="pawn"
        onmouseleave={onmouseup.clone()}
        onmousedown={onmousedown}
        onmouseup={onmouseup}
        onmousemove={onmousemove}
        style={set_position()}>
         <img src="img/pawn.png" width="50px" height="50px" />
        </div>

    }
}
