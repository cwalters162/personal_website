use yew::{html, function_component, Html};



#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div>
                <span>
                    { "© 2022. Personal Website of Cherokee Walters." }
                </span>
            </div>
        </footer>
    }
}