use yew::prelude::*;
use crate::components::{
    tech_card::TechCard,
};


#[function_component(HomePage)]
pub fn home() -> Html {
    let _about_body: Vec<String> = vec![
        "My name is Cherokee. It's nice for you to visit.".to_string(),
        "I'm a full stack developer that has been coding full time since summer 2021!
        I've been working on multiple commercial project for various goverment organisations such as Joint Special Operations Center's Global Analytics Platform,
        and XVIII Airborne Corps Data Warfare Company.".to_string(),
        "If you have more questions please send me a message on my LinkedIn!".to_string(),
    ];

    html! {
        <div class="grid">
            <div class="s0 m3 l3"/>
            <article class="s12 m6 l6">
                <h5>{"Welcome"}</h5>
                <p><i>{"waving_hand"}</i> {"Thanks for coming to my website!"}</p>
                <p>{"Links to my GitHub and LinkedIn profiles on the top right."}</p>
                <p>{"You can find more about me below."}</p>
                <p>{"Thanks again for visiting!"}</p>
            </article>
            <div class="s0 m2 l3"/>
            <div class="s0 m2 l3"/>
            <TechCard />
        </div>
    }
}