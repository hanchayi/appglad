use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, HtmlElement};
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
        let on_click = ctx.link().callback(|e: MouseEvent| {
            let div = e.target().expect("no target");
            let into = div.unchecked_into::<HtmlElement>();
            console_log!("div: {:?}", into.get_attribute("data"));
        });

        html! {
            <div class="h-full" onclick={on_click}>
                { "AssetList click name to use" }
                { assets.into_iter().map(|asset| {
                    html!{ <div class="cursor-pointer" data-id={asset} a="1"> { asset } </div>}
                }).collect::<Html>()}
            </div>
        }
    }
}