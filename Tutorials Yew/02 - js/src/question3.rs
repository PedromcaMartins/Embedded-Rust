use yew::prelude::*;

#[function_component]
pub fn Question3() -> Html {
    let input_state = use_state(|| "".to_string());

    let lion = {
        let input_state = input_state.clone();
        Callback::from(move |_| {
            input_state.set("https://duckduckgo.com/i/9ce505eb27c21438.jpg".to_string());
        })
    };

    let frog = {
        let input_state = input_state.clone();
        Callback::from(move |_| {
            input_state.set("https://duckduckgo.com/i/3a8d8019.jpg".to_string());
        })
    };

    html! {
        <>
            <div>
                <button onclick={lion}>{ "Lion" }</button>
                <button onclick={frog}>{ "Frog" }</button>
            </div>
            <br/>
            <img src={(*input_state).clone()} />
        </>
    }
}