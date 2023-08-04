pub mod crab;

use crate::components::layout::Layout;
use crab::Crab;
use rand::Rng;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

const MIN_CRABS: i32 = 15;
const MAX_CRABS: i32 = 21;

#[function_component(App)]
pub fn app() -> Html {
    let remove_amount = use_counter(1);
    let crabs = use_state(|| {
        let num_crabs = rand::thread_rng().gen_range(MIN_CRABS..=MAX_CRABS);
        (0..num_crabs).map(Crab::new).collect::<Vec<Crab>>()
    });

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
        let crabs = crabs.clone();
        let remove_amount = remove_amount.clone();

        Callback::from(move |_| {
            let mut crabs_vec = crabs.to_vec();
            let mut removed_count = 0;

            for crab in crabs_vec.iter_mut() {
                if !crab.removed {
                    crab.to_removed();
                    removed_count += 1;
                }

                if removed_count >= *remove_amount as usize {
                    break;
                }
            }

            crabs.set(crabs_vec);
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
                    { crabs.iter().map(|el: &Crab| el.html.clone()).collect::<Vec<Html>>() }
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
