use std::rc::Rc;
use yew::{Component, Context, Html, html};
use yewdux::prelude::*;
use crate::State;

pub struct IncrementComponent {
    state: Rc<State>,
    dispatch: Dispatch<State>,
}

pub enum Msg {
    /// Message to receive new state.
    State(Rc<State>),
}
impl Component for IncrementComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<State>::subscribe(ctx.link().callback(Msg::State));
        Self {
            state: dispatch.get(),
            dispatch,
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
        let onclick = self.dispatch.reduce_mut_callback(|s| s.count += 1);
        html! {
            <>
            <button onclick={onclick}>{"add 1"}</button>
            <div>{"IncrementComponent store: "}{self.state.count}</div>
            </>
        }
    }
}