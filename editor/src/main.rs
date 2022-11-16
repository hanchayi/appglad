use yew::prelude::*;
use yew_router::prelude::*;

mod utils;
mod components;
mod pages;
use pages::{
    page_not_found::PageNotFound,
    editor_page::EditorPage,
};
use yew::html::Scope;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    EditorPage,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {
    ToggleNavbar,
}

pub struct Main {
    navbar_active: bool,
}
impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <div class="ag">
                    { self.view_nav(ctx.link()) }

                    <main>
                        <Switch<Route> render={Switch::render(switch)} />
                    </main>
                </div>
            </BrowserRouter>
        }
    }
}
impl Main {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::EditorPage}>
                            { "EditorPage" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::NotFound}>
                            { "PageNotFound" }
                        </Link<Route>>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::EditorPage => {
            html! { <EditorPage /> }
        }
       
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

fn main() {
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Main>();
}