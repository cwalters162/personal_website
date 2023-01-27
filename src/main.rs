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
            <body class="light">
            <Header />
                <main class="responsive">
                    <Switch<Route> render={switch} />
                </main>
            <Footer />
            </body>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
