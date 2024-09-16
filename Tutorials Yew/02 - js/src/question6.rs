use yew::prelude::*;

#[function_component]
pub fn Question6() -> Html {
    let frog_emojis = use_state(|| "".to_string());

    let frog = {
        let frog_emojis = frog_emojis.clone();
        Callback::from(move |_| {
            frog_emojis.set(format!("{}🐸", *frog_emojis))
        })
    };

    html! {
        <>
            <img onclick={frog} src="https://duckduckgo.com/i/3a8d8019.jpg"/>
            <br/>

            <span>{(*frog_emojis).clone()}</span>
        </>
    }
}