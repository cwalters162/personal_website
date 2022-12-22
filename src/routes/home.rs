use yew::prelude::*;

use crate::components::card::Card;

#[function_component(HomePage)]
pub fn home() -> Html {
    let about_body: Vec<String> = vec![
    "This is a card that holds information about me. I am adding a lot more content just to see what happens to the card. I hope that it does what it should.".to_string(),
    "It should be showing this in two paragraphs. It should be showing this in two paragraphs. It should be showing this in two paragraphs. It should be showing this in two paragraphs. It should be showing this in two paragraphs. It should be showing this in two paragraphs. It should be showing this in two paragraphs. It should be showing this in two paragraphs. It should be showing this in two paragraphs. It should be showing this in two paragraphs. It should be showing this in two paragraphs. It should be showing this in two paragraphs.".to_string()];


    html! {
        <Card title="About" body={about_body}/>
    }
}