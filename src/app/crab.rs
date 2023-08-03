use yew::prelude::*;

#[derive(Clone)]
pub struct Crab {
    pub html: Html,
    pub id: i32,
}

impl Crab {
    pub fn new(id: i32) -> Self {
        let crab_html = html! {
            <img
             class="mr-3 inline cursor-pointer lg:w-[75px] lg:h-[75px]"
             src="/public/cuddlyferris.svg"
             alt="crabimg"
             width="60"
             height="60"
            />
        };

        Crab {
            id,
            html: crab_html,
        }
    }
}
