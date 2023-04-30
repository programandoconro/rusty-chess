use yew::prelude::*;
use yew::{html, Callback};

struct Coordinates {
    x: Option<i32>,
    y: Option<i32>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub width: i32,
    pub height: i32,
    pub src: String,
}

#[function_component]
pub fn Piece(props: &Props) -> Html {
    let coordinates: UseStateHandle<Coordinates> = use_state(|| Coordinates { x: None, y: None });
    let is_dragging: UseStateHandle<bool> = use_state(|| false);
    let z_index: UseStateHandle<i32> = use_state(|| 99);

    let onmousedown: Callback<MouseEvent> = {
        let is_dragging: UseStateHandle<bool> = is_dragging.clone();
        let z_index = z_index.clone();
        Callback::from(move |event: MouseEvent| -> () {
            event.prevent_default();
            is_dragging.set(true);
            z_index.set(1000);
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
        let props = props.clone();

        Callback::from(move |event: MouseEvent| -> () {
            event.prevent_default();

            if *is_dragging == true {
                let x = ((event.page_x()) as f64 / props.width as f64) * 100.0;

                let y = ((event.page_y()) as f64 / props.height as f64) * 100.0;

                //web_sys::console::log_1(&y.to_string().into());
                coordinates.set(Coordinates {
                    x: Some(x as i32),
                    y: Some(y as i32),
                });
            }
        })
    };

    let set_position = || -> Option<String> {
        match coordinates.x.is_some() && coordinates.y.is_some() {
            true => Some(format!(
                "left: {}%; top: {}%; zindex: {}",
                coordinates.x.unwrap(),
                coordinates.y.unwrap(),
                *z_index
            )),
            false => None,
        }
    };

    html! {

        <img
            class="piece" src={props.src.clone()}
            style={set_position()}
            onmouseleave={onmouseup.clone()}
            onmousedown={onmousedown}
            onmouseup={onmouseup}
            onmousemove={onmousemove}
         />

    }
}
