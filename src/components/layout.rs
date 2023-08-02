use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &Props) -> Html {
    html! {
        <div class="flex items-center justify-center h-screen flex-col">
            {props.children.clone()}
        </div>
    }
}
