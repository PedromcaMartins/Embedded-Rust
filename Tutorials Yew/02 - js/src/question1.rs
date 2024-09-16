use gloo_dialogs::alert;
use yew::prelude::*;

#[function_component]
pub fn Question1() -> Html {
    let console_hi = Callback::from(move |_| {
        let greeting = "hi there".to_string();
        log::info!("{}", greeting);
    });

    let alert_hi = Callback::from(move |_| {
        let greeting = "hi there";
        alert(greeting);
    });

    html! {
        <div>
            <button onclick={console_hi}>{ "Console Hi" }</button>
            <button onclick={alert_hi}>{ "Alert Hi" }</button>
        </div>
    }
}