use yew::prelude::*;

#[derive(Clone)]
enum Animals {
    Lion,
    Frog,
    Invalid,
    None
}

impl Animals {
    fn src(&self) -> String {
        match self {
            Self::Lion => "https://duckduckgo.com/i/9ce505eb27c21438.jpg".to_string(),
            Self::Frog => "https://duckduckgo.com/i/3a8d8019.jpg".to_string(),
            Self::Invalid => "".to_string(),
            Self::None => "".to_string(),
        }
    }

    fn result_invalid_command(&self) -> String {
        match self {
            Self::Lion => "".to_string(),
            Self::Frog => "".to_string(),
            Self::Invalid => "Invalid Command".to_string(),
            Self::None => "".to_string(),
        }
    }
}

#[function_component]
pub fn Question7() -> Html {
    let animal = use_state(|| Animals::None);

    let oninput = {
        let animal = animal.clone();

        Callback::from(move |e: InputEvent| {
            let input_value = e.data();
            log::info!("{:?}", input_value);
            match input_value.as_deref() {
                Some("display lion") => animal.set(Animals::Lion),
                Some("display frog") => animal.set(Animals::Frog),
                _ => animal.set(Animals::Invalid),
            }
        })
    };

    html! {
        <>
            <div>
                {"Enter command: "}
                <input type="text" placeholder="Enter a command..." {oninput} />
            </div>
            <br/>

            <img src={animal.src()} width="200" height="200" />
            <br/>

            <span>{animal.result_invalid_command()}</span>
        </>
    }
}