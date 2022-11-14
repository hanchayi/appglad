use yew::{html, Component, Context, Html};

mod components;

pub mod utils;

use components::topbar::Topbar;
use components::canvas::Canvas;

struct Main {
}

impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="ag-editor">
                <div class="top">
                    <Topbar />
                </div>
                <div class="left">{ "left" }</div>
                <div class="middle">
                    <Canvas />
                </div>
                <div class="right">{ "right" }</div>
            </div>
            
        }
    }

    
}

impl Main {
    
}


fn main() {
    yew::start_app::<Main>();
}