use std::rc::Rc;
use yew::{Component, Context, Html, html};
use yewdux::prelude::*;
use crate::State;

pub struct ViewComponent {
    state: Rc<State>
}

pub enum Msg {
    /// Message to receive new state.
    State(Rc<State>),
}
impl Component for ViewComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<State>::subscribe(ctx.link().callback(Msg::State));
        Self {
            state: dispatch.get()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::State(state) => {
                self.state = state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let count = self.state.count;
        html! {
            <h1>{ count }</h1>
        }
    }
}