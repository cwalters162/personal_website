use yew::{html, function_component, Html};



#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="fixed">
                <p class="center-align">{"Personal Website of Cherokee Walters."}</p>
        </footer>
    }
}