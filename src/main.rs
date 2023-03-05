use yew::prelude::*;
mod board;
mod piece;
use piece::Pawn;

#[function_component]
fn App() -> Html {
    let is_flipped = use_state(|| false);

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

    html! {
        <div class={".container"}>
        <table>
            {
                for board::create_board(*is_flipped).iter().map(render_squares).rev()
            }
        </table>
        <Pawn />

        <aside>
        <button onclick={onclick}>{"Flip"}</button>
        </aside>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
