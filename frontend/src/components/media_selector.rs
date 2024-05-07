use yew::html::onclick::Event;
use yew::{function_component, html, use_state, Callback, Html, Properties};

#[derive(Debug, Clone, PartialEq)]
pub enum MediaSelectorOption {
    Movies,
    TvShows,
    YTChannel,
    All,
    None,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MediaSelectorProps {
    #[prop_or(MediaSelectorOption::None)]
    pub default_option: MediaSelectorOption,
    pub on_change: Callback<MediaSelectorOption>,
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(MediaSelector)]
pub fn media_selector(props: &MediaSelectorProps) -> Html {
    let props_clone = props.to_owned();
    let selected_value = use_state(|| props.default_option.clone());
    let selected_value_clone = selected_value.clone();
    let is_default = |option: MediaSelectorOption| option == *selected_value_clone;

    let update_value = move |opt: MediaSelectorOption| {
        let props_clone = props.to_owned();
        let selected_value_clone = selected_value.clone();
        Callback::from(move |_: Event| {
            selected_value_clone.set(opt.clone());
            props_clone.on_change.emit(opt.clone());
        })
    };

    html! {
        <div class="join flex justify-center">
            <input
                class="join-item btn"
                type="radio"
                name="options"
                aria-label="All"
                checked={is_default(MediaSelectorOption::All)}
                disabled={props_clone.disabled}
                //onclick={Callback::from(&move |_| update(MediaSelectorOption::Both))}
                onclick={update_value(MediaSelectorOption::All)}
            />
            <input
                class="join-item btn grow"
                type="radio"
                name="options"
                aria-label="Movies"
                checked={is_default(MediaSelectorOption::Movies)}
                disabled={props_clone.disabled}
                //onclick={Callback::from(&move |_| update(MediaSelectorOption::Movies))}
                onclick={update_value(MediaSelectorOption::Movies)}
            />
            <input
                class="join-item btn grow"
                type="radio"
                name="options"
                aria-label="Tv Shows"
                checked={is_default(MediaSelectorOption::TvShows)}
                disabled={props_clone.disabled}
                //onclick={Callback::from(&move |_| update(MediaSelectorOption::TvShows))}
                onclick={update_value(MediaSelectorOption::TvShows)}
            />
            <input
                class="join-item btn grow"
                type="radio"
                name="options"
                aria-label="YT Channels"
                checked={is_default(MediaSelectorOption::YTChannel)}
                disabled={props_clone.disabled}
                //onclick={Callback::from(&move |_| update(MediaSelectorOption::TvShows))}
                onclick={update_value(MediaSelectorOption::YTChannel)}
            />
        </div>
    }
}
