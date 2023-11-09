mod increment_component;
mod view_component;

use crate::increment_component::IncrementComponent;
use yew::prelude::*;
use yewdux::prelude::*;
use crate::view_component::ViewComponent;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct State {
    count: u32,
}

struct AppComponent {}

impl Component for AppComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <ViewComponent />
            <IncrementComponent />
            </>
        }
    }
}

pub fn main() {
    yew::Renderer::<AppComponent>::new().render();
}
