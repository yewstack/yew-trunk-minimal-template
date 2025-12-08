use yew::prelude::*;

#[component]
pub fn App() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.svg" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
