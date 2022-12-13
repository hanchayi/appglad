use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, HtmlElement, DragEvent};
use yew::{Component, Context, html, Html, Properties};

use crate::console_log;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub on_hover: Callback<Hovered>,
    pub name: String,
}



pub struct AssetItem {
}


impl Component for AssetItem {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmouseover = {
            let name = ctx.props().name.clone();
            ctx.props.on_hover.reform(|e: MouseEvent| {
                e.stop_propagation();
                console_log!("mouse over");
                Hovered
            })
        };

        html!{ <div class="cursor-pointer" data-id={asset} a="1" draggable="true" { onmouseover }> { asset } </div>}
    }
}