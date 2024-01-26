use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub title: String,
    pub body: Vec<String>,
}

#[function_component(Card)]
pub fn card(CardProps { title, body }: &CardProps ) -> Html {
    let card_body: Html = body.iter().map(|text| html! {
        <p>{text}</p>
    }).collect();

    html! {
        <div class="card">
            <h1>{title}</h1>
            {card_body}
        </div>
    }
}