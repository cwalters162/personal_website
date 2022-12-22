use yew::{Html, html, function_component};
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav>
            <div class="title">
                <span class="name"> {"Cherokee Walters"}</span>
                <span class="sub-name"> {"Full Stack Software Developer"}</span>
            </div>
            <div class="links">
                <Link<Route> to={Route::Home} >{"Home"}</Link<Route>>
                <a class="center" href="https://www.github.com">{ "GitHub" }</a>
                <Link<Route> to={Route::NotFound} >{"Not Found"}</Link<Route>>
            </div>
        </nav>
    }
}