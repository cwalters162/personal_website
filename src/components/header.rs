use yew::{Html, html, function_component};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="fill small-elevate max">
            <nav>
                <h5 class="center-align">{"Cherokee Walters"}</h5>
                <h1 class="max"> </h1>
                <a class="circle transparent" href="https://github.com/cwalters162" target="_blank" rel="noopener noreferrer" >
                    <img height="32" width="32" src="img/github-logo-black.svg" />
                    <div class="tooltip bottom">{"GitHub"}</div>
                </a>
                <a style="" class="square" href="https://www.linkedin.com/in/cwalters162" target="_blank" rel="noopener noreferrer" >
                  <img height="32" width="32" src="img/linkedin-logo-black.svg" />
                  <div class="tooltip bottom">{"LinkedIn"}</div>
                </a>
            </nav>
        </header>
    }
}