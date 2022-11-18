use yew::{html, Component, Context, Html};
use crate::{components::{ layout::Layout }};

pub struct EditorPage {
}

impl Component for EditorPage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Layout />
        }
    }
}



