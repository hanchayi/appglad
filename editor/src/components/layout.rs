use yew::{html, Component, Context, Html};
use crate::{components::{ topbar::Topbar, canvas::Canvas, asset_list::AssetList }, console_log};

pub struct Layout {
}

impl Component for Layout {
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
                <div class="left">
                    <AssetList />
                </div>
                <div class="middle">
                    <Canvas />
                </div>
                <div class="right">{ "right" }</div>
            </div>
            
        }
    }
}

impl Layout {
  
}


