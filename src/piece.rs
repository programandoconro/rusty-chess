use yew::prelude::*;
struct Coordinates {
    x: i32,
    y: i32,
}

#[function_component]
pub fn Pawn() -> Html {
    let coordinates: UseStateHandle<Coordinates> = use_state(|| Coordinates { x: 0, y: 0 });
    let ondragend = {
        let coordinates = coordinates.clone();
        Callback::from(move |event: DragEvent| -> () {
            coordinates.set(Coordinates {
                x: event.client_x(),
                y: event.client_y(),
            });
        web_sys::console::log_1(&coordinates.x.to_string().into());
            web_sys::console::log_1(&coordinates.y.to_string().into());
        })
    };

    html! {
        <div ondrag={ondragend.clone()} ondragend= {ondragend} draggable={"true"} class="pawn"
        style={format!("left: {}px; top: {}px;",
         coordinates.x, coordinates.y)}>

        <div class="head"></div>

        <div class="neck">
            <div class="neck-block"></div>
            <div class="space-down"></div>
        </div>

        <div class="torso"> </div>
        <div class="space-down"></div>
        <div class="space-down"></div>
        
        <div class="legs"> </div>
        <div class="space-down"></div>
        
        <div class="base"> </div>

        </div>

    }
}
