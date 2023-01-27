use web_sys::EventTarget;
use web_sys::HtmlImageElement;
use wasm_bindgen::JsCast;
use gloo_console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[function_component(TechCard)]
pub fn tech_card() -> Html {

    fn on_linux_mouse_over(e: MouseEvent) {
        let target: Option<EventTarget> = e.target();
        let img = target.and_then(|t| t.dyn_into::<HtmlImageElement>().ok());
        if let Some(img) = img {
            img.set_src("img/linux-logo-color.svg");
            let object = JsValue::from(img.src());
            log!(object)
        }
    }
    fn on_linux_mouse_out(e: MouseEvent) {
        let target: Option<EventTarget> = e.target();
        let img = target.and_then(|t| t.dyn_into::<HtmlImageElement>().ok());
        if let Some(img) = img {
            img.set_src("img/linux-logo-black.svg");
            let object = JsValue::from(img.src());
            log!(object)
        }
    }

    html! {
        <article class="s12 m8 l6">
            <h5>{"Technologies I Use"}</h5>
            <div class="grid">
                <span class="s12 m12 l12">{"Operating Systems"}</span>
                <div class="s2 m2 l2 center-align">
                    <img id="linux-img" height="64" width="64"
                        src="img/linux-logo-black.svg"
                        onmouseover={|e| on_linux_mouse_over(e)}
                        onmouseout={|e| on_linux_mouse_out(e)}
                    />
                    <div class="tooltip bottom">{"GNU/Linux"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/macos-logo-black.svg" />
                    <div class="tooltip bottom">{"MacOS"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/windows-logo-black.svg" />
                    <div class="tooltip bottom">{"Windows"}</div>
                </div>
                <span class="s12 m12 l12">{"Programming Languages"}</span>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/html5-logo-black.svg" />
                    <div class="tooltip bottom">{"HTML5"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/css3-logo-black.svg" />
                    <div class="tooltip bottom">{"CSS3"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/javascript-logo-black.svg" />
                    <div class="tooltip bottom">{"JavaScript"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/python-logo-black.svg" />
                    <div class="tooltip bottom">{"Python"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/rust-logo-black.svg" />
                    <div class="tooltip bottom">{"Rust"}</div>
                </div>
                <span class="s12 m12 l12">{"Frameworks"}</span>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/azure-logo-black.svg" />
                    <div class="tooltip bottom">{"Microsoft Azure"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/docker-logo-black.svg" />
                    <div class="tooltip bottom">{"Docker"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/flask-logo-black.svg" />
                    <div class="tooltip bottom">{"Flask"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/react-logo-black.svg" />
                    <div class="tooltip bottom">{"React"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/yew-logo.svg" />
                    <div class="tooltip bottom">{"Yew"}</div>
                </div>
                <span class="s12 m12 l12">{"Databases"}</span>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/mysql-logo-black.svg" />
                    <div class="tooltip bottom">{"MySQL"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/postgresql-logo-black.svg" />
                    <div class="tooltip bottom">{"PostgreSQL"}</div>
                </div>
                <span class="s12 m12 l12">{"Developer Tools"}</span>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/jetbrains-logo-black.svg" />
                    <div class="tooltip bottom">{"JetBrains"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/git-logo-black.svg" />
                    <div class="tooltip bottom">{"Git"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/gitea-logo-black.svg" />
                    <div class="tooltip bottom">{"Gitea"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/github-logo-black.svg" />
                    <div class="tooltip bottom">{"GitHub"}</div>
                </div>
                <div class="s2 m2 l2 center-align">
                    <img height="64" width="64" src="img/gitlab-logo-black.svg" />
                    <div class="tooltip bottom">{"GitLab"}</div>
                </div>
            </div>
        </article>
    }
}