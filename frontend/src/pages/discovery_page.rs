use std::io::Write;
use std::ops::Deref;
use crate::api::collections_api::api_patch_user_collection;
use crate::api::discovery_api::{
    api_get_discovery_both_random, api_get_discovery_movies_random,
    api_get_discovery_tv_shows_random,
};
use crate::components::header::Header;
use crate::components::media_selector::{ MediaSelectorOption, MediaSelector};
use crate::components::spinner::Spinner;
use crate::router;
use crate::store::{set_page_loading, set_show_alert, Store};
use common::model::collections::{Media, UserCollection};
use futures::FutureExt;
use gloo::console::console;
use serde::{Deserialize, Serialize};
use validator::Validate;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use yew_router::hooks::use_navigator;
use yewdux::functional::use_store;
use crate::api::tmdb_api;

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

// TODO: Could be DRY'd out.
fn media_card(
    media: Media,
    yes_callback: &Callback<MouseEvent>,
    no_callback: &Callback<MouseEvent>,
    skip_callback: &Callback<MouseEvent>,
) -> Html {
    let options_fragment = html! {
        <div class="card-actions justify-around pt-4">
            <button class="basis-1/4 btn btn-outline btn-error" onclick={no_callback}>
               <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path fill="currentColor"
                    d="M9 11C9.55228 11 10 10.5523 10 10C10 9.44772 9.55228 9 9 9C8.44772 9 8 9.44772 8 10C8 10.5523 8.44772 11 9 11Z"
                  />
                  <path fill="currentColor"
                    d="M14 17C14 15.8954 13.1046 15 12 15C10.8954 15 10 15.8954 10 17H8C8 14.7909 9.79086 13 12 13C14.2091 13 16 14.7909 16 17H14Z"
                  />
                  <path fill="currentColor"
                    d="M16 10C16 10.5523 15.5523 11 15 11C14.4477 11 14 10.5523 14 10C14 9.44772 14.4477 9 15 9C15.5523 9 16 9.44772 16 10Z"
                  />
                  <path fill-rule="evenodd" clip-rule="evenodd" fill="currentColor"
                    d="M22 12C22 17.5228 17.5228 22 12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12ZM20 12C20 16.4183 16.4183 20 12 20C7.58172 20 4 16.4183 4 12C4 7.58172 7.58172 4 12 4C16.4183 4 20 7.58172 20 12Z"
                  />
               </svg>
               // {"👎"}
            </button>
            <button class="btn btn-ghost" onclick={skip_callback}>
                {"Skip"}
            </button>
            <button class="basis-1/4 btn btn-outline btn-success" onclick={yes_callback}>
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path fill="currentColor"
                    d="M12 17C14.2091 17 16 15.2091 16 13H8C8 15.2091 9.79086 17 12 17Z"
                  />
                  <path fill="currentColor"
                    d="M10 10C10 10.5523 9.55228 11 9 11C8.44772 11 8 10.5523 8 10C8 9.44772 8.44772 9 9 9C9.55228 9 10 9.44772 10 10Z"
                  />
                  <path fill="currentColor"
                    d="M15 11C15.5523 11 16 10.5523 16 10C16 9.44772 15.5523 9 15 9C14.4477 9 14 9.44772 14 10C14 10.5523 14.4477 11 15 11Z"
                  />
                  <path fill-rule="evenodd" clip-rule="evenodd" fill="currentColor"
                    d="M22 12C22 17.5228 17.5228 22 12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12ZM20 12C20 16.4183 16.4183 20 12 20C7.58172 20 4 16.4183 4 12C4 7.58172 7.58172 4 12 4C16.4183 4 20 7.58172 20 12Z"
                  />
                </svg>
                // {"👍"}
            </button>
        </div>
    };

    match media {
        Media::Movie(movie) => {
            match movie.details {
                None => {
                    html!(
                        <div class="text-center border border-base-content bg-base-200 card image-full">
                            //<figure><img style="transform: scale(1.25);" class="p-0" src={fig_path} /></figure>
                            <div class="card-body">
                                <h2 class="card-title">
                                    <div>{movie.name.to_string()}</div>
                                    <div class="text-xs">{format!("({})", movie.year.to_string())}</div>
                                </h2>
                                <p class="h-64"></p>
                                {options_fragment}
                            </div>
                        </div>
                    )
                }
                Some(details) => {
                    let backdrop_path = details.backdrop_path.unwrap_or_default();
                    let fig_path = format!("https://image.tmdb.org/t/p/w500{backdrop_path}");
                    html!(
                        <div class="text-center border border-base-content bg-base-200 max-w-prose card image-full">
                            <figure><img style="transform: scale(1.25);" class="p-0" src={fig_path} /></figure>
                            <div class="card-body">
                                <h2 class="card-title">
                                    <div>{movie.name.to_string()}</div>
                                    <div class="text-xs">{format!("({})", movie.year.to_string())}</div>
                                </h2>
                                <p class="overflow-y-auto h-64">{details.overview}</p>
                                {options_fragment}
                            </div>
                        </div>
                    )
                }
            }
        }
        Media::TvShow(tv_show) => {
            match tv_show.details {
                None => {
                    html!(
                        <div class="text-center border border-base-content bg-base-200 card image-full">
                            //<figure><img style="transform: scale(1.25);" class="p-0" src={fig_path} /></figure>
                            <div class="card-body">
                                <h2 class="card-title">
                                    <div>{tv_show.name}</div>
                                    <div class="text-xs">{format!("({})", tv_show.first_air_date)}</div>
                                </h2>
                                <p class="h-64"></p>
                                {options_fragment}
                            </div>
                        </div>
                    )
                }
                Some(details) => {
                    let backdrop_path = details.backdrop_path.unwrap_or_default();
                    let fig_path = format!("https://image.tmdb.org/t/p/w500{backdrop_path}");
                    html!(
                        <div class="text-center border border-base-content bg-base-200 max-w-prose card image-full">
                            <figure><img style="transform: scale(1.25);" class="p-0" src={fig_path} /></figure>
                            <div class="card-body">
                                <h2 class="card-title">
                                    <div>{tv_show.name}</div>
                                    <div class="text-xs">{format!("({})", tv_show.first_air_date)}</div>
                                </h2>
                                <p class="overflow-y-auto h-64">{details.overview}</p>
                                {options_fragment}
                            </div>
                        </div>
                    )
                }
            }
        }
    }
}

#[function_component(DiscoveryPage)]
pub fn discovery_page() -> Html {

    let (store, dispatch) = use_store::<Store>();
    let navigator = use_navigator().unwrap();
    if store.auth_user.is_none() {
        navigator.push(&router::Route::LoginPage);
    }
    let tmdb_key = store
        .auth_user
        .as_ref()
        .unwrap()
        .tmdb_api_key
        .clone()
        .expect("No TMDB API Key");
    let count = 15_i16; // How many Titles we're requesting from the discovery API.
    let discovery_queue = use_state(|| Vec::<Media>::new().to_vec());
    let collections = use_state(|| store.collections.clone().unwrap_or_else(std::vec::Vec::new));
    let media_selector_value = use_state(|| MediaSelectorOption::Both);

    let do_discovery = |_| {
        let collections = collections.clone();
        let discovery_queue = discovery_queue.clone();
        let media_selector_value = media_selector_value.clone();
        let dispatch = dispatch.clone();

        Callback::from(move |event: MouseEvent| {
            let tk = tmdb_key.to_string();
            let discovery_queue = discovery_queue.clone();
            let collections = collections.clone();
            let cloned_dispatch = dispatch.clone();
            let msv = media_selector_value.clone();
            wasm_bindgen_futures::spawn_local(async move {
                set_page_loading(true, &cloned_dispatch);
                let discovery_type = match *msv {
                    MediaSelectorOption::Movies => api_get_discovery_movies_random(Some(count)).await,
                    MediaSelectorOption::TvShows => api_get_discovery_tv_shows_random(Some(count)).await,
                    MediaSelectorOption::Both => api_get_discovery_both_random(Some(count)).await,
                    MediaSelectorOption::None => api_get_discovery_both_random(Some(count)).await ,
                };

                match discovery_type {
                    Ok(cols) => {
                        set_page_loading(false, &cloned_dispatch);
                        let out = match tmdb_api::tmdb_coalesce_media(tk.as_str(), &cols).await {
                            Ok(media) => media,
                            Err(e) => {
                                console!(format!("Error Coalescing with TMDB: {}", e));
                                cols.to_vec()
                            }
                        };

                        discovery_queue.set(out);
                        set_page_loading(false, &cloned_dispatch);
                    }
                    Err(e) => {
                        console!(e.clone());
                        //set_page_loading(false, &cloned_dispatch);
                        if e.contains("You are not logged in") {
                            set_show_alert(e.to_string(), &cloned_dispatch);
                            //navigator.push(&router::Route::LoginPage);
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

        Callback::from(move |event: MouseEvent| {
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

    let media_selector_value_clone = media_selector_value.clone();
    let media_selector_change: Callback<MediaSelectorOption> = Callback::from(move |option: MediaSelectorOption| {
        media_selector_value_clone.set(option);
    });

    html! {
        <>
            <Header />
            <section class="grid justify-items-stretch justify-center">
                <div class="grid lg:w-[65vw]">
                    <div class="w-3/5 justify-self-center">
                        <div class="text-center pb-2">
                            <h2 class="text-3xl font-bold">{"Discovery Queue"}</h2>
                        </div>
                        <div class="flex flex-col pb-2">
                            <MediaSelector
                                default_option={MediaSelectorOption::Both}
                                on_change={media_selector_change}
                            />
                            <div class="flex gap-2">
                                <input class="input input-bordered join-item grow" placeholder="(Optional) Custom Query"/>
                                <div class="btn join-item" onclick={do_discovery("")}
                                    disabled={store.page_loading}>
                                    {"Discover"}
                                </div>
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
                        <div class="stack w-4/5 grid justify-stretch justify-self-center">
                            {
                                discovery_queue.iter().map(|media| {
                                    html!({
                                        media_card(
                                            media.clone(),
                                            &do_rating(
                                                &media,
                                                DiscoveryRatingOption::UpVote
                                            ),
                                            &do_rating(
                                                &media,
                                                DiscoveryRatingOption::DownVote
                                            ),
                                            &do_rating(
                                                &media,
                                                DiscoveryRatingOption::Skip
                                            ),
                                        )
                                    })
                                }).collect::<Html>()
                            }
                        </div>
                    }
                </div>
            </section>
        </>
    }
}

// fn get_card(media: Media) -> Html {
//     match media {
//         Media::Movie(movie) => {
//             html!({
//                 movie_card(
//                     movie.clone(),
//                     &on_click(
//                         movie.name.to_string(),
//                         movie.year,
//                         DiscoveryRatingOption::UpVote
//                     ),
//                     &on_click(
//                         movie.name.to_string(),
//                         movie.year,
//                         DiscoveryRatingOption::DownVote
//                     ),
//                     &on_click(
//                         movie.name.to_string(),
//                         movie.year,
//                         DiscoveryRatingOption::Skip
//                     ),
//                 )
//             })
//         }
//         Media::TvShow(tv_show) => {
//             html!(<p>{"Farts"}</p>)
//         }
//     }
// }
