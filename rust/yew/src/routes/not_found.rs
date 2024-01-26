use yew::prelude::*;

use crate::components::card::Card;

#[function_component(NotFoundPage)]
pub fn not_found() -> Html {
    let not_found_body: Vec<String> = vec![
    "The resource you were looking for was not found.".to_string(),
    "Please try looking somewhere else!".to_string()];


    html! {
        <Card title="Resource Not Found" body={not_found_body}/>
    }
}