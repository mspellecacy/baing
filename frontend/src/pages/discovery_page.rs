use crate::api::collections_api::api_patch_user_collection;
use crate::api::discovery_api::{api_get_discovery_both_random, api_get_discovery_movies_random, api_get_discovery_tv_shows_random, api_get_discovery_yt_channels_random};
use crate::api::{tmdb_api, coalesce_media};
use crate::components::figures::{FaceFrown, FaceSmile};
use crate::components::media_card::MediaCard;
use crate::components::media_selector::{MediaSelector, MediaSelectorOption};
use crate::components::spinner::Spinner;
use crate::router;
use crate::store::{set_page_loading, set_show_alert, Store};
use crate::ui_helpers::get_value_from_input_by_id;
use common::model::collections::{Media, UserCollection};
use common::model::core::{DiscoveryMeta, Movie};
use gloo::console::console;
use serde::{Deserialize, Serialize};
use validator::Validate;
use yew::prelude::*;
use yew::{function_component, html, Html};
use yew_router::hooks::use_navigator;
use yewdux::functional::use_store;

#[derive(Debug, Clone)]
enum DiscoveryRatingOption {
    UpVote,
    DownVote,
    Skip,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct DiscoverySchema {
    media: String,
    custom_query: String,
}

#[function_component(DiscoveryPage)]
pub fn discovery_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let navigator = use_navigator().unwrap();

    let mut tmdb_key: Option<String> = None;
    {
        if let Some(user) = store.auth_user.clone() {
            if let Some(api_key) = &user.tmdb_api_key {
                let key = api_key.clone();
                tmdb_key = Some(key);
            } else {
                navigator.push(&router::Route::ProfilePage);
            }
        }

        match tmdb_key {
            None => navigator.push(&router::Route::LoginPage),
            Some(_) => (),
        }
    }
    // TODO: Make this a user-choice w/ a dropdown. 5~50
    // How many Titles we're requesting from the discovery API.
    let count = 15_i16; // How many Titles we're requesting from the discovery API.
    let discovery_queue = use_state(|| Vec::<Media>::new());
    let collections = use_state(|| store.collections.clone().unwrap_or_default());
    let media_selector_option = use_state(|| MediaSelectorOption::All);

    // Feels ridiculous walking these values down scope? Im not understanding something.
    let do_discovery = {
        let discovery_queue = discovery_queue.clone();
        let media_selector_value = media_selector_option.clone();
        let dispatch = dispatch.clone();
        let navigator = navigator.clone();
        let key = tmdb_key.unwrap().clone();

        Callback::from(move |_: MouseEvent| {
            discovery_queue.set(Vec::new()); // reset the discovery queue
            let tk = key.to_string();
            let discovery_queue = discovery_queue.clone();
            let dispatch = dispatch.clone();
            let media_selector_value = media_selector_value.clone();
            let nav = navigator.clone();
            let query =
                get_value_from_input_by_id("#discovery_custom_query").unwrap_or(String::from(""));

            wasm_bindgen_futures::spawn_local(async move {
                set_page_loading(true, &dispatch);
                let discovery = match *media_selector_value {
                    MediaSelectorOption::Movies => {
                        api_get_discovery_movies_random(Some(count), &query).await
                    }
                    MediaSelectorOption::TvShows => {
                        api_get_discovery_tv_shows_random(Some(count), &query).await
                    }
                    MediaSelectorOption::YTChannel => {
                        api_get_discovery_yt_channels_random(Some(count), &query).await
                    }
                    // Both & None
                    _ => api_get_discovery_both_random(Some(count), &query).await,
                };

                match discovery {
                    Ok(discovered) => {
                        set_page_loading(false, &dispatch);

                        let out =
                            match coalesce_media(tk.as_str(), &discovered).await {
                                Ok(mut media) => media,
                                Err(e) => {
                                    console!(format!("Error Coalescing with TMDB: {}", e));
                                    discovered.to_vec()
                                }
                            };

                        discovery_queue.set(out);
                        set_page_loading(false, &dispatch);
                    }
                    Err(e) => {
                        console!(e.clone());
                        //set_page_loading(false, &cloned_dispatch);
                        if e.contains("You are not logged in") {
                            set_show_alert(e.to_string(), &dispatch);
                            nav.push(&router::Route::LoginPage);
                        }
                    }
                }
            });
        })
    };

    let do_rating = |media: &Media, rating: DiscoveryRatingOption| {
        let discovery_queue = discovery_queue.clone();
        let collections = collections.clone();
        let media = media.clone();

        Callback::from(move |_: MouseEvent| {
            let media_clone = media.clone();
            if let Some(media) = discovery_queue.get(0) {
                let cols = collections.clone();
                let dq = discovery_queue.clone();

                // Get a mutable of the local UC we're updating
                let mut uc: UserCollection = match rating {
                    DiscoveryRatingOption::UpVote => {
                        let mut tu_col = cols
                            .iter()
                            .filter(|uc| uc.special.clone().is_some_and(|s| s == "thumbsup"))
                            .collect::<Vec<&UserCollection>>();

                        tu_col
                            .get_mut(0)
                            .expect("Missing Thumbs Up Collection?")
                            .to_owned()
                    }
                    DiscoveryRatingOption::DownVote => {
                        let mut td_id = cols
                            .iter()
                            .filter(|uc| uc.special.clone().is_some_and(|s| s == "thumbsdown"))
                            .collect::<Vec<&UserCollection>>();

                        td_id
                            .get_mut(0)
                            .expect("Missing Thumbs Down Collection?")
                            .to_owned()
                    }
                    DiscoveryRatingOption::Skip => {
                        let mut sk_id = cols
                            .iter()
                            .filter(|uc| uc.special.clone().is_some_and(|s| s == "skipped"))
                            .collect::<Vec<&UserCollection>>();

                        sk_id
                            .get_mut(0)
                            .expect("Missing Skipped Collection?")
                            .to_owned()
                    }
                };

                // Push our new item into our mutable collection
                uc.collection.entries.push(media.clone());

                // Get a Mutable of Vec<UserCollection> without the UC we've updated.
                let mut new_cols: Vec<UserCollection> = collections[..]
                    .iter()
                    .filter(|c| c.id != uc.id)
                    .map(|c| c.to_owned())
                    .collect();

                // Patch our updated collections to the backend.
                wasm_bindgen_futures::spawn_local(async move {
                    match api_patch_user_collection(uc.clone()).await {
                        Ok(collection) => {
                            new_cols.push(collection);
                            cols.set(new_cols.clone());

                            // deref 'copy' into an array slice w/o the rated media
                            let new_queue: Vec<_> = dq[..]
                                .iter()
                                .map(|m| m.to_owned())
                                .filter(|m| m != &media_clone)
                                .collect();

                            // Update our discovery queue.
                            dq.set(new_queue);
                        }
                        Err(e) => {
                            console!(format!("{e:?}"));
                        }
                    }
                });
            }
        })
    };

    let on_change_media_selector: Callback<MediaSelectorOption> = {
        let media_selector_option = media_selector_option.clone();
        Callback::from(move |option: MediaSelectorOption| {
            media_selector_option.set(option);
        })
    };

    enum ShuffleDirection {
        Right,
        Left,
    }

    let on_shuffle = |dir: ShuffleDirection| {
        let discovery_queue = discovery_queue.clone();
        Callback::from(move |_: MouseEvent| {
            let mut rot_queue = discovery_queue.to_vec();
            match dir {
                ShuffleDirection::Right => rot_queue.rotate_right(1),
                ShuffleDirection::Left => rot_queue.rotate_left(1),
            }
            discovery_queue.set(rot_queue);
        })
    };

    html! {
        <section class="grid justify-items-stretch justify-center place-content-center">
            <div class="grid lg:w-[65vw]">
                <div class="w-4/5 justify-self-center">
                    <div class="text-center pb-2">
                        <h2 class="text-3xl font-bold">{"Discovery Queue"}</h2>
                    </div>
                    <div class="flex flex-col pb-2 gap-2">
                        <input
                            id="discovery_custom_query"
                            class="input input-bordered"
                            placeholder="(Optional) Query"
                            disabled={store.page_loading}
                        />
                        <MediaSelector
                            default_option={MediaSelectorOption::All}
                            on_change={on_change_media_selector}
                            disabled={store.page_loading}
                        />
                        <div
                            class="btn flex-none"
                            onclick={do_discovery}
                            disabled={store.page_loading}>
                            {"Discover"}
                        </div>
                    </div>
                </div>
                if discovery_queue.len() == 0 {
                    <div class="stack w-4/5 grid justify-stretch justify-self-center">
                        <div class="text-center border border-base-content bg-base-200 card image-full">
                            <div class="card-body">
                                <h2 class="card-title place-content-center">
                                    if store.page_loading {
                                        <Spinner />
                                    } else {
                                        {"Hi! Your discovery queue is empty. 😟"}
                                    }
                                </h2>
                            </div>
                        </div>
                    </div>
                } else {
                //<div class="w-full place-content-center">
                    <div class="stack w-4/5 grid justify-stretch justify-self-center">
                        {
                            discovery_queue.iter().map(|media| {
                                html!{
                                    <MediaCard media={media.to_owned()}>
                                        <div class="card-actions justify-around pt-4">
                                            <a class="btn btn-ghost" onclick={on_shuffle(ShuffleDirection::Left)}>
                                                {"❮"}
                                            </a>
                                            <button
                                                class="basis-1/4 btn btn-outline btn-error"
                                                onclick={do_rating(media, DiscoveryRatingOption::DownVote)}>
                                                <FaceFrown />
                                            </button>
                                            <button
                                                class="btn btn-ghost"
                                                onclick={do_rating(media, DiscoveryRatingOption::Skip)}>
                                                {"Skip"}
                                            </button>
                                            <button
                                                class="basis-1/4 btn btn-outline btn-success"
                                                onclick={do_rating(media, DiscoveryRatingOption::UpVote)}>
                                                <FaceSmile />
                                            </button>
                                            <a class="btn btn-ghost" onclick={on_shuffle(ShuffleDirection::Right)}>
                                                {"❯"}
                                            </a>
                                        </div>
                                    </MediaCard>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                //</div>
                }
            </div>
        </section>
    }
}
