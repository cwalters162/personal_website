mod components;
mod routes;

//Internal
use components::{
    header::Header,
    footer::Footer
};
use routes::{switch, Route};
//External
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <div class="content">
                <Switch<Route> render={switch} />
            </div>
            <Footer />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
