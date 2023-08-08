use crate::minimax::Player;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameResultProps {
    pub winner: Player,
}

#[function_component(GameResult)]
pub fn game_result(props: &GameResultProps) -> Html {
    html! {
        <div>
            {match props.winner {
                Player::User => "user won",
                Player::Computer => "computer won",
            }}
        </div>
    }
}
