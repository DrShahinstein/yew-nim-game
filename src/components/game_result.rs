use crate::minimax::Player;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameResultProps {
    pub winner: Player,
    pub on_replay: Callback<MouseEvent>,
}

#[function_component(GameResult)]
pub fn game_result(props: &GameResultProps) -> Html {
    let on_replay_click = props.on_replay.clone();

    html! {
        <div class="flex flex-col items-center justify-center h-screen bg-gradient-to-b from-purple-500 to-blue-600">
            <h2 class="text-4xl mb-6 text-white">
                {format!("{} Won!", match props.winner {
                    Player::User => "You",
                    Player::Computer => "Computer",
                })}
            </h2>
            <button
                onclick={on_replay_click}
                class="bg-gradient-to-r from-yellow-300 to-yellow-500 hover:from-yellow-400 hover:to-yellow-600 text-gray-800 font-bold py-3 px-6 rounded-full shadow-lg transform hover:scale-105 transition-transform duration-300">
                {"Play Again"}
            </button>
        </div>
    }
}
