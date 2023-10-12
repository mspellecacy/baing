use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SpinnerProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Spinner)]
pub fn spinner_component(props: &SpinnerProps) -> Html {
    let classes = classes!("loading", "loading-infinity", props.class.clone());

    html! {
        <span class={classes}></span>
    }
}
