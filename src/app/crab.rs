use yew::prelude::*;

#[derive(Clone)]
pub struct Crab {
    pub html: Html,
    pub id: i32,
    pub removed: bool,
}

impl Crab {
    pub fn new(id: i32) -> Self {
        let html = html! {
            <img
             class="mr-3 inline cursor-pointer lg:w-[75px] lg:h-[75px]"
             src="/public/cuddlyferris.svg"
             alt="crabimg"
             width="60"
             height="60"
            />
        };

        Crab {
            html,
            id,
            removed: false,
        }
    }

    pub fn to_removed(&mut self) {
        let html = html! {
            <img
             class="mr-3 inline cursor-pointer lg:w-[75px] lg:h-[75px] removed-crab"
             src="/public/cuddlyferris.svg"
             alt="crabimg"
             width="60"
             height="60"
            />
        };

        self.html = html;
        self.removed = true;
    }
}
