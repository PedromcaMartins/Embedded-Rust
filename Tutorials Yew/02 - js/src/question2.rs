use yew::prelude::*;

#[function_component]
pub fn Question2() -> Html {
    let input_state = use_state(|| "".to_string());

    let lion = {
        let input_state = input_state.clone();
        Callback::from(move |_| {
            input_state.set("The Lion buttom is clicked".to_string());
        })
    };

    let frog = {
        let input_state = input_state.clone();
        Callback::from(move |_| {
            input_state.set("The Frog buttom is clicked".to_string());
        })
    };

    html! {
        <>
            <div>
                <button onclick={lion}>{ "Lion" }</button>
                <button onclick={frog}>{ "Frog" }</button>
            </div>
            <br/>
            <input value={(*input_state).clone()} />
        </>
    }
}