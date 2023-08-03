use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLESHEET: &str = include_str!("../app.css");

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(Layout)]
pub fn layout(props: &Props) -> Html {
    let css: Style = Style::new(STYLESHEET).unwrap();

    html! {
        <div class={css}>
            <div class="layout">
                {props.children.clone()}
            </div>
        </div>
    }
}
