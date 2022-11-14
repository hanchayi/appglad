use yew::{Component, Context, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Topbar;

impl Component for Topbar {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Topbar
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="h-full">{ "topbar1" }</div>
        }
    }
}