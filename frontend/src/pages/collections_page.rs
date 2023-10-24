use crate::api::collections_api::api_get_user_collections;
use crate::components::media_selector::{MediaSelector, MediaSelectorOption};
use crate::router;
use crate::store::{set_page_loading, set_show_alert, Store};
use common::model::collections::{Media, UserCollection};
use gloo::console::console;
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
    let media_selector_option = use_state(|| MediaSelectorOption::Both);

    {
        let collections = collections.clone();
        let dispatch = dispatch.clone();
        use_effect_with_deps(
            move |_| {
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
            },
            (),
        );
    }

    let media_filter = |media: &Media, option: &MediaSelectorOption| -> bool {
        match option {
            MediaSelectorOption::Movies => matches!(media, &Media::Movie(_)),
            MediaSelectorOption::TvShows => matches!(media, &Media::TvShow(_)),
            _ => true, // Both & None
        }
    };

    let active_collection: &[UserCollection] = &collections[..];
    let media_selector_option_clone = media_selector_option.clone();
    let on_change_media_selector: Callback<MediaSelectorOption> =
        Callback::from(move |option: MediaSelectorOption| {
            media_selector_option_clone.set(option);
        });

    html! {
        <section class="grid justify-items-stretch justify-center">
            <div class="grid lg:w-[65vw]">
                <div class="w-3/5 justify-self-center">
                    <div class="text-center pb-2">
                        <h2 class="text-3xl font-bold">{"Collections"}</h2>
                    </div>
                        <div class="flex flex-col pb-2">
                            <MediaSelector
                                default_option={MediaSelectorOption::Both}
                                on_change={on_change_media_selector}
                            />
                        </div>
                    </div>
                <div class="grow flex flex-cols justify-center">
                    if active_collection.is_empty() {
                        <p>{"You have no collections! You should create one!"}</p>
                    } else {
                        {
                            active_collection.iter()
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
                                    <div class="card">
                                        <div class="card-body">
                                        <h2 class="card-title">{ format!("{} ({})",col.name, col.collection.entries.len()) }</h2>
                                        <ul>
                                            {
                                                col.collection.entries.iter()
                                                //.filter(|m| media_filter(m, &media_selector_option))
                                                .map(|media| {
                                                    media_item(media.clone())
                                                }).collect::<Html>()
                                            }
                                        </ul>
                                        </div>
                                    </div>
                                }
                            })
                            .collect::<Html>()
                        }
                    }
                </div>
            </div>
        </section>
    }
}
