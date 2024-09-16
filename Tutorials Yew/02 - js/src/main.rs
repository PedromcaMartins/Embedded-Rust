use yew::prelude::*;

mod question1;
use question1::Question1;
mod question2;
use question2::Question2;
mod question3;
use question3::Question3;
mod question4;
use question4::Question4;
mod question5;
use question5::Question5;
mod question6;
use question6::Question6;
mod question7;
use question7::Question7;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Question1/>
            <br/><br/>

            <Question2/>
            <br/><br/>

            <Question3/>
            <br/><br/>

            <Question4/>
            <br/><br/>

            <Question5/>
            <br/><br/>

            <Question6/>
            <br/><br/>

            <Question7/>
            <br/><br/>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}