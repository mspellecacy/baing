use yew::{Callback, function_component, html, Html, Properties, use_state};
use yew::html::onclick::Event;

#[derive(Debug, Clone, PartialEq)]
pub enum MediaSelectorOption {
    Movies,
    TvShows,
    Both,
    None
}

#[derive(Clone, PartialEq, Properties)]
pub struct MediaSelectorProps {
    #[prop_or(MediaSelectorOption::None)]
    pub default_option: MediaSelectorOption,
    pub on_change: Callback<MediaSelectorOption>,

}

#[function_component(MediaSelector)]
pub fn media_selector(props: &MediaSelectorProps) -> Html {
    let props_clone = props.to_owned();
    let selected_value = use_state(|| props.default_option.clone());
    let selected_value_clone = selected_value.clone();

    let is_default = |option: MediaSelectorOption| {
      option == *selected_value_clone
    };

    let update_value = move |opt: MediaSelectorOption| {
        let props_clone = props.to_owned();
        let selected_value_clone = selected_value.clone();
        Callback::from(move |_: Event| {
            selected_value_clone.set(opt.clone());
            props_clone.on_change.emit(opt.clone());
        })
    };

    html! {
        <>
        <div class="join pb-2 flex justify-center">
            <input
                class="join-item btn grow"
                type="radio"
                name="options"
                aria-label="Movies"
                checked={is_default(MediaSelectorOption::Movies)}
                //onclick={Callback::from(&move |_| update(MediaSelectorOption::Movies))}
                onclick={update_value(MediaSelectorOption::Movies)}
            />
            <input
                class="join-item btn"
                type="radio"
                name="options"
                aria-label="Both"
                checked={is_default(MediaSelectorOption::Both)}
                //onclick={Callback::from(&move |_| update(MediaSelectorOption::Both))}
                onclick={update_value(MediaSelectorOption::Both)}
            />
            <input
                class="join-item btn grow"
                type="radio"
                name="options"
                aria-label="Tv Shows"
                checked={is_default(MediaSelectorOption::TvShows)}
                //onclick={Callback::from(&move |_| update(MediaSelectorOption::TvShows))}
                onclick={update_value(MediaSelectorOption::TvShows)}
            />
        </div>
        </>
    }
}