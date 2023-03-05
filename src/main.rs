use yew::prelude::*;
mod board;

struct Coordinates {
    x: i32,
    y: i32,
}
#[function_component]
fn App() -> Html {
    let is_flipped = use_state(|| false);
    let coordinates: UseStateHandle<Coordinates> = use_state(|| Coordinates { x: 0, y: 0 });

    let onclick: Callback<MouseEvent> = {
        let is_flipped = is_flipped.clone();
        web_sys::console::log_1(&is_flipped.to_string().into());
        Callback::from(move |_| match *is_flipped {
            true => is_flipped.set(false),
            false => is_flipped.set(true),
        })
    };

    let render_squares = |squares: &Vec<String>| -> Html {
        html! {
            <tr>
                {
                    for squares.iter().map(|square| html!{
                        <td><p>{square}</p></td>
                    })
                }
            </tr>
        }
    };

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
        <div class={".container"}>
        <table>
            {
                for board::create_board(*is_flipped).iter().map(render_squares).rev()
            }
        </table>
        <div ondrag={ondragend.clone()} ondragend= {ondragend} draggable={"true"} class="pawn"
        style={format!("left: {}px; top: {}px;",
         coordinates.x, coordinates.y)}></div>

        <aside>
        <button onclick={onclick}>{"Flip"}</button>
        </aside>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
