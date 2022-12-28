use yew::prelude::*;

use crate::components::card::Card;

#[function_component(HomePage)]
pub fn home() -> Html {
    let welcome_body: Vec<String> = vec![
        "Hello! Thanks for coming to my website!".to_string(),
        "Links to my GitHub and LinkedIn profiles on the top right.".to_string(),
        "You can find more about me below.".to_string(),
        "Thanks again for visiting!".to_string()
    ];

    let about_body: Vec<String> = vec![
        "My name is Cherokee. It's nice for you to visit.".to_string(),
        "I'm a full stack developer that has been coding full time since summer 2021!
        I've been working on multiple commercial project for various goverment organisations such as Joint Special Operations Center's Global Analytics Platform,
        and 18 Airborne Corps Data Warfare Company.".to_string(),
        "If you have more questions please send me a message on my LinkedIn!".to_string(),
    ];


    html! {
        <>
            <Card title="Welcome" body={welcome_body}/>
            <Card title="About Me" body={about_body}/>
        </>
    }
}