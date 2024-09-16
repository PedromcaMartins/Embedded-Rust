use yew::prelude::*;

#[function_component]
pub fn Question4() -> Html {
    let sound = use_state(|| "".to_string());
    let text_color = use_state(|| "".to_string());

    let lion = {
        let sound = sound.clone();
        let text_color = text_color.clone();
        Callback::from(move |_| {
            sound.set("Roar".to_string());
            text_color.set("orange".to_string())
        })
    };

    let frog = {
        let sound = sound.clone();
        let text_color = text_color.clone();
        Callback::from(move |_| {
            sound.set("Ribbit".to_string());
            text_color.set("green".to_string())
        })
    };

    html! {
        <>
            <img onclick={lion} src="https://duckduckgo.com/i/9ce505eb27c21438.jpg"/>
            <img onclick={frog} src="https://duckduckgo.com/i/3a8d8019.jpg"/>
            <br/>

            <span color={(*text_color).clone()}>{(*sound).clone()}</span>
        </>
    }
}