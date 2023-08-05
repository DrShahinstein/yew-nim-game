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
             class="crab"
             src="/public/cuddlyferris.svg"
             alt="crabimg"
             width="75"
             height="75"
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
             class="crab removed-crab"
             src="/public/cuddlyferris.svg"
             alt="crabimg"
             width="75"
             height="75"
            />
        };

        self.html = html;
        self.removed = true;
    }
}
