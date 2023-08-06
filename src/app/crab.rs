use yew::prelude::*;

pub enum AnimationRotation {
    Left,
    Right,
}

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

    pub fn turn_into_removed(&mut self, animation_rotation: AnimationRotation) {
        let rotation = match animation_rotation {
            AnimationRotation::Left => "removed-crab-sliding-left",
            AnimationRotation::Right => "removed-crab-sliding-right",
        };

        let html = html! {
            <img
             class={format!("crab removed-crab {}", rotation)}
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
