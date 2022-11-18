use yew::{html, Component, Context, Html};
use crate::{components::{ topbar::Topbar, canvas::Canvas }, console_log};

pub struct EditorPage {
}

impl Component for EditorPage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_click = ctx.link().callback(|_| {
            console_log!("on click1")
        });

        html! {
            <div class="ag-editor">
                <div class="top" onclick={ on_click }>
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


