use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, HtmlElement, DragEvent};
use yew::{Component, Context, html, Html, Properties};

use crate::console_log;

#[derive(PartialEq, Properties)]
pub struct Props {
}



pub struct AssetList {
}


impl Component for AssetList {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        AssetList {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let assets = vec!["img", "div"];
        let onclick = ctx.link().callback(|e: MouseEvent| {
            let div = e.target().expect("no target");
            let into = div.unchecked_into::<HtmlElement>();
            console_log!("div: {:?}", into.get_attribute("data"));
        });

        let onmouseover = ctx.link().callback(|e: MouseEvent| {
            e.stop_propagation();
            console_log!("mouse move");
        });


        html! {
            <div class="h-full" {onclick}>
                <div class="select-none">
                    { "AssetList click name to use" }
                </div>
                { assets.into_iter().map(|asset| {
                    html!{ <div class="cursor-pointer" data-id={asset} a="1" draggable="true" { onmouseover }> { asset } </div>}
                }).collect::<Html>()}
            </div>
        }
    }
}