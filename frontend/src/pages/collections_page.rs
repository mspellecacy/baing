use crate::api::collections_api::api_get_user_collections;
use crate::components::media_card::MediaCard;
use crate::components::media_selector::{MediaSelector, MediaSelectorOption};
use crate::router;
use crate::store::{set_page_loading, set_show_alert, Store};
use common::model::collections::{Media, UserCollection};
use gloo::console::console;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::functional::use_store;

#[derive(Clone, Debug, PartialEq)]
struct ContentCollections {
    collections: Vec<String>,
}

fn media_item(media: Media) -> Html {
    let li = match media {
        Media::Movie(movie) => format!("📽️ {movie}"),
        Media::TvShow(tv_show) => format!("📺 {tv_show}"),
        Media::YTChannel(yt_channel) => format!("▶️ {yt_channel}")
    };

    html! {<li>{li}</li>}
}

#[function_component(CollectionsPage)]
pub fn collections_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let navigator = use_navigator().unwrap();
    let user = store.auth_user.clone();
    if user.is_none() {
        navigator.push(&router::Route::LoginPage);
    }
    let collections = use_state(|| store.collections.clone().unwrap_or_default());
    let media_selector_option = use_state(|| MediaSelectorOption::All);
    let active_col: UseStateHandle<Option<UserCollection>> = use_state(|| None);

    {
        let collections = collections.clone();
        let dispatch = dispatch.clone();
        use_effect_with((), move |_| {
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                set_page_loading(true, &dispatch);
                let response = api_get_user_collections().await;
                match response {
                    Ok(cols) => {
                        set_page_loading(false, &dispatch);
                        collections.set(cols.clone());
                        dispatch.reduce_mut(move |store| {
                            store.collections = Some(cols);
                        })
                    }
                    Err(e) => {
                        console!(format!("Error getting user collections: {e}"));
                        set_page_loading(false, &dispatch);
                        if e.contains("You are not logged in") {
                            set_show_alert(e.to_string(), &dispatch);
                            navigator.push(&router::Route::LoginPage);
                        }
                    }
                }
            });
            || ()
        });
    }

    let media_filter = |media: &Media, option: &MediaSelectorOption| -> bool {
        match option {
            MediaSelectorOption::Movies => matches!(media, &Media::Movie(_)),
            MediaSelectorOption::TvShows => matches!(media, &Media::TvShow(_)),
            _ => true, // Both & None
        }
    };

    let on_change_media_selector: Callback<MediaSelectorOption> = {
        let media_selector_option = media_selector_option.clone();
        Callback::from(move |option: MediaSelectorOption| {
            media_selector_option.set(option);
        })
    };

    let on_change_collection_selector = {
        let collections = collections.clone();
        let active_col = active_col.clone();
        Callback::from(move |event: Event| {
            let selected = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
                .unwrap()
                .value();
            let col_id = Uuid::parse_str(&selected).unwrap();
            let col_search = collections.iter().find(|c| c.id == col_id);
            match col_search {
                None => active_col.set(None),
                Some(col) => active_col.set(Some(col.to_owned())),
            }
        })
    };

    let on_media_click = {
        Callback::from(move |event: MouseEvent| {
            console!("You clicked! Huzzah for you!".to_string());
        })
    };

    html! {
        <section class="grid justify-items-stretch justify-center">
            <div class="grid lg:w-[65vw] sm:w-[95wv]">
                <div class="lg:w-3/5 justify-self-center">
                    <select
                        id="bng_Collection_Selector"
                        class="select select-bordered select-lg w-full mt-1"
                        onchange={on_change_collection_selector}>
                        // <option>Option 1</option>
                        <option disabled={true} selected={true}>{"Collections..."}</option>
                        {
                            collections.iter().map(|col|{
                                let name = format!("{} ({})",col.name, col.collection.entries.len());

                                html!{<option value={col.id.to_string()}>{name}</option>}
                            }).collect::<Html>()
                        }
                    </select>
                </div>
                <div class="lg:w-3/5 grow flex flex-col justify-self-center justify-center">
                    <div class="grow overflow-hidden flex-col">
                        if active_col.clone().is_some() {
                            {
                                active_col.iter()
                                .map(|col| {
                                    let mut filtered_col = col.clone();
                                    filtered_col.collection.entries = col.collection.entries.iter()
                                        .filter(|m| media_filter(m, &media_selector_option))
                                        .map(|m| m.to_owned())
                                        .collect();

                                    filtered_col
                                })
                                .map(|col| {
                                    html!{
                                    <>
                                        <div class="grow pb-2 pt-2">
                                            <MediaSelector
                                                default_option={MediaSelectorOption::All}
                                                on_change={&on_change_media_selector}
                                            />
                                        </div>
                                        {
                                            col.collection.entries
                                            .iter()
                                            .map(|media| {
                                                html!{
                                                    <div class="pt-2">
                                                        <MediaCard
                                                            media={media.clone()}
                                                            lite={true}  // TODO: Make user-toggle
                                                            // onclick={on_media_click}
                                                        />
                                                    </div>
                                                }
                                            }).collect::<Html>()
                                        }

                                    </>
                                    }
                                })
                                .collect::<Html>()
                            }
                        } else {
                           <p>{"... no collection selected ..."}</p>
                        }
                    </div>
                    // if active_collection.is_empty() {
                    //     <p>{"You have no collections! You should create one!"}</p>
                    // } else {
                    //     {
                    //         active_collection.iter()
                    //         .map(|col| {
                    //             let mut filtered_col = col.clone();
                    //             filtered_col.collection.entries = col.collection.entries.iter()
                    //                 .filter(|m| media_filter(m, &media_selector_option))
                    //                 .map(|m| m.to_owned())
                    //                 .collect();
                    //
                    //             filtered_col
                    //         })
                    //         .map(|col| {
                    //             html!{
                    //                 <div class="card">
                    //                     <div class="card-body">
                    //                     <h2 class="card-title">{ format!("{} ({})",col.name, col.collection.entries.len()) }</h2>
                    //                     <ul>
                    //                         {
                    //                             col.collection.entries.iter()
                    //                             //.filter(|m| media_filter(m, &media_selector_option))
                    //                             .map(|media| {
                    //                                 media_item(media.clone())
                    //                             }).collect::<Html>()
                    //                         }
                    //                     </ul>
                    //                     </div>
                    //                 </div>
                    //             }
                    //         })
                    //         .collect::<Html>()
                    //     }
                    // }
                </div>
            </div>
        </section>
    }
}
