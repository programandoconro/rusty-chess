use yew::prelude::*;

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
            //web_sys::console::log_1(&is_dragging.to_string().into());
            if *is_dragging == true {
                coordinates.set(Coordinates {
                    x: event.x(), 
                    y: event.y()
                });
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
