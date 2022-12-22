pub mod home;

use crate::routes::home::HomePage;
use yew::prelude::*;
use yew_router::prelude::*;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<HomePage />},
        Route::NotFound => html! { <p> {"Page Not Found"} </p> }
    }
}