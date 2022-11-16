use yew::{html, Component, Context, Html};
use crate::components::{ topbar::Topbar, canvas::Canvas };

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

impl EditorPage {
    
}


