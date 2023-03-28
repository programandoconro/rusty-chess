use yew::prelude::*;
mod board;
mod piece;
use piece::Pawn;
use web_sys::{ MouseEvent};
use yew::{html, Callback};

#[function_component]
fn App() -> Html {
    let is_flipped: UseStateHandle<bool> = use_state(|| false);
    let onclick: Callback<MouseEvent> = {
        let is_flipped: UseStateHandle<bool> = is_flipped.clone();
        Callback::from(move |_| match *is_flipped {
            true => is_flipped.set(false),
            false => is_flipped.set(true),
        })
    };

    let set_pawn = |square: &String| -> Html {
        if square.ends_with("7") {
            html! {
               <Pawn />
            }
        } else {
            html! {}
        }
    };

    let render_squares = |squares: &Vec<String>| -> Html {
        html! {
            <tr>
                {
                    for squares.iter().map(|square|
                    html!{
                        <td class="square"><p class="nomenclature">{square} </p> {set_pawn(square)}</td>
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
        <aside>
        <button onclick={onclick}>{"Flip"}</button>
        </aside>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
