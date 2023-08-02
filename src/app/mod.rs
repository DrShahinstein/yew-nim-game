use crate::components::layout::Layout;
use rand::Rng;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

const MIN_CRABS: i32 = 15;
const MAX_CRABS: i32 = 21;

#[function_component(App)]
pub fn app() -> Html {
    let crabs_count = use_counter(rand::thread_rng().gen_range(MIN_CRABS..=MAX_CRABS));
    let remove_amount = use_counter(1);

    let on_set_removal = {
        let remove_amount = remove_amount.clone();

        Callback::from(move |input_event: InputEvent| {
            if let Some(input_element) = input_event
                .target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            {
                let removal_value: i32 = match input_element.value().parse() {
                    Ok(value) => value,
                    _ => 1,
                };

                remove_amount.set(removal_value.min(2).max(1));
            }
        })
    };

    let on_remove_click = {
        let crabs_count = crabs_count.clone();
        let remove_amount = remove_amount.clone();

        Callback::from(move |_| {
            let removal = *crabs_count - *remove_amount;

            crabs_count.set(removal);

            if removal <= 0 {
                println!("You won");
                /* TODO: Implement win situation for player */
            }

            /* TODO: Make counter move using Minimax algorithm as player two (computer) */
        })
    };

    html! {
        <Layout>
            /*
            TODO: Create a pleasent title with a correct position on the window
            <div class="text-5xl font-bold text-center text-indigo-600">
                {"RUST NIM GAME"}
            </div>
            */

            <div>
                <div class="ml-10 mr-10">
                    { (0..*crabs_count).map(render_crab).collect::<Vec<Html>>() }
                </div>
                <div class="flex items-center justify-center mt-5">
                    <input
                        class="border rounded-lg px-4 py-2 w-40 lg:w-48 text-center focus:outline-none focus:ring focus:border-blue-300 transition text-gray-800 border-gray-400"
                        type="number"
                        min="1"
                        max="2"
                        value={remove_amount.to_string()}
                        oninput={on_set_removal} />
                    <button
                        onclick={on_remove_click}
                        class="bg-blue-500 hover:bg-blue-600 text-white rounded-lg px-4 py-2 ml-2 focus:outline-none focus:ring focus:border-blue-300 transition"
                        type="button">
                        {"Remove Crabs"}
                    </button>
                </div>
            </div>
        </Layout>
    }
}

fn render_crab(index: i32) -> Html {
    html! {
        <img
            class="mr-3 inline cursor-pointer lg:w-[75px] lg:h-[75px]"
            src="/public/cuddlyferris.svg"
            alt="crabimg"
            width="60"
            height="60"
            key={format!("crab-{}", index)} />
    }
}
