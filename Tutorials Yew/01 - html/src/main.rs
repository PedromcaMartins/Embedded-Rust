use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <h1>{ "Pedro Martins, SN: 8757914" }</h1>

            <h3>
                <span style="color: orange;">{"World "}</span>
                <span style="color: blue;">{"Gurning "}</span>
                <span style="color: purple;">{"Competition "}</span>
                <span>{"🗿"}</span>
            </h3>

            <table>
                <tr>
                    <th>{"Ranking"}</th>
                    <th>{"Team"}</th>
                    <th>{"Members"}</th>
                </tr>

                <tr>
                    <td>{"1st"}</td>
                    <td>
                        <a href="http://didd.gov/">{"Kingdom of Didd"}</a>
                    </td>
                    <td>
                        <ul>
                            <li>
                                <b>{"Thing 1 "}</b>
                                <i>{"(team leader)"}</i>
                            </li>
                            <li>{"North-Going Zax"}</li>
                            <li>{"Cindy-Lou Who"}</li>
                            <li>{"McMonkey McBean"}</li>
                            <li>{"Vlad Vlad-i-Koff"}</li>
                        </ul>
                    </td>
                </tr>

                <tr>
                    <td>{"2nd"}</td>
                    <td>
                        <a href="http://binn.gov/">{"Kingdom of Binn"}</a>
                    </td>
                    <td>
                        <ul>
                            <li>
                                <b>{"Thing 2 "}</b>
                                <i>{"(team leader)"}</i>
                            </li>
                            <li>{"South-Going Zax"}</li>
                            <li>{"The Grinch"}</li>
                            <li>{"Bartholomew Cubbins"}</li>
                            <li>{"Fizza-ma Wizza-ma Dill"}</li>
                        </ul>
                    </td>
                </tr>
            </table>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}