use yew::prelude::*;
mod board;
mod piece;
use piece::Pawn;
use web_sys::{HtmlElement, MouseEvent};
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

    let table_ref = use_node_ref();
    let width = use_state(|| 1);
    let height = use_state(|| 1);

    {
        let table_ref = table_ref.clone();
        let width = width.clone();
        let height = height.clone();
        use_effect_with_deps(
            move |table_ref| {
                let table = table_ref
                    .cast::<HtmlElement>()
                    .expect("div_ref not attached to div element");

                width.set(table.client_width());
                height.set(table.client_height());
            },
            table_ref,
        );
    }

    let set_pawn = |square: &String| -> Html {
        if square.ends_with("7") {
            html! {
                <>
               <Pawn width={*width} height={*height} />
               </>
            }
        } else {
            html! {<></>}
        }
    };

    let render_squares = |squares: &Vec<String>| -> Html {
        html! {
            <tr>
                {
                    for squares.iter().map(|square|
                    html!{
                        <td ><p class="nomenclature">{square} </p> {set_pawn(square)}</td>
                    })
                }
            </tr>
        }
    };

    html! {
        <div >
                <button onclick={onclick}>{"Flip"}</button>

            <table ref={table_ref}>
                <tbody>
                    {
                    for board::create_board(*is_flipped).iter().map(render_squares).rev()
                    }
                </tbody>
            </table>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
