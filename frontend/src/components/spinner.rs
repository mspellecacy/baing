use yew::prelude::*;

#[function_component(Spinner)]
pub fn spinner_component() -> Html {
    html! {
        <span class="loading loading-infinity loading-xl"></span>
    }
}
