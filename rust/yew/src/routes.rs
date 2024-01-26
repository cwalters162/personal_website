pub mod home;
pub mod not_found;

use crate::routes::home::HomePage;
use crate::routes::not_found::NotFoundPage;
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
        Route::NotFound => html! { <NotFoundPage /> }
    }
}