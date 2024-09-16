use yew::prelude::*;

#[function_component]
pub fn Question5() -> Html {
    let lion_counter = use_state(|| 0);
    let frog_counter = use_state(|| 0);

    let lion = {
        let lion_counter = lion_counter.clone();
        Callback::from(move |_| {
            lion_counter.set(*lion_counter + 1);
        })
    };

    let frog = {
        let frog_counter = frog_counter.clone();
        Callback::from(move |_| {
            frog_counter.set(*frog_counter + 1);
        })
    };

    html! {
        <>
            <img onclick={lion} src="https://duckduckgo.com/i/9ce505eb27c21438.jpg"/>
            <img onclick={frog} src="https://duckduckgo.com/i/3a8d8019.jpg"/>
            <br/><br/>

            <span color={"orange"}>{format!("Number of lion clicks: {}", (*lion_counter).clone())}</span>
            <br/>
            <span color={"green"}> {format!("Number of frog clicks: {}", (*frog_counter).clone())}</span>
            <br/>
        </>
    }
}