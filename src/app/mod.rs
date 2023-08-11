pub mod crab;

use crate::components::game_result::GameResult;
use crate::components::layout::Layout;
use crate::minimax::best_move;
use crate::minimax::Player;
use crab::AnimationRotation;
use crab::Crab;
use gloo::timers::callback::Timeout;
use rand::Rng;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

const MIN_CRABS: i32 = 15;
const MAX_CRABS: i32 = 21;

#[derive(Clone, Copy)]
struct GameState {
    winner: Option<Player>,
    show_result: bool,
}
impl GameState {
    pub fn new() -> Self {
        Self {
            winner: None,
            show_result: false,
        }
    }
}

pub fn initialize_crabs() -> Vec<Crab> {
    let num_crabs = rand::thread_rng().gen_range(MIN_CRABS..=MAX_CRABS);
    (0..num_crabs).map(Crab::new).collect::<Vec<Crab>>()
}

pub fn get_remaining_crabs(crabs_vec: Vec<Crab>) -> i32 {
    crabs_vec.iter().filter(|crab| !crab.removed).count() as i32
}

#[function_component(App)]
pub fn app() -> Html {
    let remove_amount = use_counter(1);
    let crabs = use_state(initialize_crabs);
    let game_result = use_state(GameState::new);

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
        let game_result = game_result.clone();
        let crabs = crabs.clone();
        let remove_amount = remove_amount.clone();

        Callback::from(move |_| {
            if game_result.winner.is_some() {
                return;
            }

            let mut crabs_vec = crabs.to_vec();
            let mut removed_crabs = 0;

            for crab in crabs_vec.iter_mut() {
                if !crab.removed {
                    crab.turn_into_removed(AnimationRotation::Left);
                    removed_crabs += 1;
                }

                if removed_crabs >= *remove_amount as usize {
                    break;
                }
            }

            let remaining_crabs = get_remaining_crabs(crabs_vec.clone());

            // User wins
            if remaining_crabs == 0 {
                game_result.set(GameState {
                    winner: Some(Player::User),
                    show_result: true,
                })
            }
            // Computer wins
            else {
                let computer_move = best_move(remaining_crabs, Player::Computer);
                let iter_mut_crabs_reversed = crabs_vec.iter_mut().rev();
                let mut removed_crabs = 0;

                for crab in iter_mut_crabs_reversed {
                    if !crab.removed && removed_crabs < computer_move {
                        crab.turn_into_removed(AnimationRotation::Right);
                        removed_crabs += 1;
                    }
                }

                crabs.set(crabs_vec.clone());

                let remaining_crabs = get_remaining_crabs(crabs_vec.clone());
                if remaining_crabs == 0 {
                    let game_result = game_result.clone();
                    let timeout = Timeout::new(1_200, move || {
                        game_result.set(GameState {
                            winner: Some(Player::Computer),
                            show_result: true,
                        });
                    });
                    timeout.forget();
                }
            }
        })
    };

    let replay_game = {
        let crabs = crabs.clone();
        let game_result = game_result.clone();

        Callback::from(move |_| {
            crabs.set(initialize_crabs());
            game_result.set(GameState::new());
        })
    };

    let game_result = game_result.clone();
    if game_result.show_result {
        let winner = game_result.winner.unwrap();
        return html! {
            <GameResult winner={winner} on_replay={replay_game.clone()} />
        };
    }

    html! {
        <Layout>
            <div class="rust-nim-game-title">
                {"RUST NIM GAME"}
            </div>

            <div class="nim-game">
                <div class="crabs-flex">
                    { crabs.iter().map(|el: &Crab| el.html.clone()).collect::<Vec<Html>>() }
                </div>
                <div class="crabs-control">
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
