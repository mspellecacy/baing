use crate::api::collections_api::api_patch_user_collection;
use crate::api::discovery_api::api_get_discovery_movies_random;
use crate::api::tmdb_api::api_tmdb_get_search_movie_details;
use crate::components::header::Header;
use crate::router;
use crate::store::{set_page_loading, set_show_alert, Store};
use common::model::collections::{Media, UserCollection};
use common::model::discovery::{Movie, MovieDetails};
use gloo::console::console;
use std::error;
use std::future::Future;
use std::ops::Deref;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use yew_router::hooks::use_navigator;
use yewdux::functional::use_store;

#[derive(Debug, Clone)]
enum DiscoveryRatingOption {
    UpVote,
    DownVote,
    Skip,
}

async fn tmdb_coalesce_movies(
    key: String,
    movies: Vec<Movie>,
) -> Result<Vec<Movie>, Box<dyn error::Error>> {
    let mut out = movies.clone();
    for (i, mut movie) in out.clone().into_iter().enumerate() {
        match api_tmdb_get_search_movie_details(key.to_string(), movie.name, movie.year).await {
            Ok(search_results) => {
                // For now we just pop the first result off the top.
                if let Some(first) = search_results.first() {
                    let details = MovieDetails::from(first.to_owned());
                    if out.first().is_some() {
                        out[i].details = Some(details.clone());
                    }
                }
            }
            Err(e) => {
                console!(format!("{:?}", e));
                return Err(e);
            }
        }
    }

    Ok(out)
}



fn movie_card(
    movie: Movie,
    yes_callback: &Callback<MouseEvent>,
    no_callback: &Callback<MouseEvent>,
    skip_callback: &Callback<MouseEvent>,
) -> Html {
    let options_fragment = html!{
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
            </button>
        </div>
    };

    match movie.details {
        None => {
            html!(
                <div class="text-center border border-base-content w-96 h-80 card bg-base-200 carousel-item">
                    <div class="card-body">
                        <h2 class="card-title">
                            <span class="indicator-item badge badge-primary">{movie.year.to_string()}</span>
                            <div>{movie.name.to_string()}</div>
                            <p class="min-h"><span class="loading loading-infinity loading-lg"></span></p>
                            {options_fragment}
                        </h2>
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
                        <p class="min-h">{details.overview}</p>
                        {options_fragment}
                    </div>
                </div>
            )
        }
    }
}

#[function_component(DiscoveryPage)]
pub fn discovery_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let navigator = use_navigator().unwrap();
    let user = store.auth_user.clone();
    if user.is_none() {
        navigator.push(&router::Route::LoginPage);
    }

    let count = 15_i16;
    let discovery_queue = use_state(|| Vec::new().to_vec());
    let collections = use_state(|| store.collections.clone().unwrap_or_else(|| vec![]));

    {
        let discovery_queue = discovery_queue.clone();
        let tmdb_key = user.unwrap().tmdb_api_key.unwrap();
        use_effect_with_deps(
            move |_| {
                let dispatch = dispatch.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    set_page_loading(true, &dispatch);
                    let response = api_get_discovery_movies_random(Some(count)).await;

                    match response {
                        Ok(cols) => {
                            let mut out = cols.clone();
                            set_page_loading(false, &dispatch);

                            let _ = match tmdb_coalesce_movies(tmdb_key, cols).await {
                                Ok(movies) => {
                                    out = movies.to_owned();
                                }
                                Err(e) => {
                                    console!(e.to_string());
                                }
                            };

                            discovery_queue.set(out);
                        }
                        Err(e) => {
                            console!(e.clone());
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

    let on_click = |movie_name: String, movie_year: i32, rating: DiscoveryRatingOption| {
        let discovery_queue = discovery_queue.clone();
        let collections = collections.clone();

        Callback::from(move |event: MouseEvent| {
            let movie_name_clone = movie_name.clone();
            if let Some(movie) = discovery_queue.get(0) {
                let cols = collections.clone();
                let dq = discovery_queue.clone();
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

                // Push our new item into the collection
                uc.collection.entries.push(Media::Movie(movie.clone()));

                // Get a Mutable of Vec<UserCollection> without the UC we've updated.
                let mut new_cols: Vec<UserCollection> = collections[..]
                    .into_iter()
                    .filter(|c| c.id != uc.id)
                    .map(|c| c.to_owned())
                    .collect();

                // Patch our updated collections to the backend.
                wasm_bindgen_futures::spawn_local(async move {
                    match api_patch_user_collection(uc.clone()).await {
                        Ok(collection) => {
                            new_cols.push(collection);
                            cols.set(new_cols.clone());

                            // deref 'copy' into an array slice
                            let new_queue: Vec<_> = dq[..]
                                .iter()
                                .map(|m| m.to_owned())
                                .filter(|m| (m.name != movie_name_clone) && (m.year != movie_year))
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

    html! {
      <>
        <Header />
            <section class="grid place-content-center items-start">
                <div class="text-center p-4">
                    <h2 class="text-3xl font-bold">{"Discovery Queue"}</h2>
                </div>
                <div class="stack">
                    {
                        discovery_queue.iter().map(|movie| {
                            html!({
                                movie_card(
                                    movie.clone(),
                                    &on_click(
                                        movie.name.to_string(),
                                        movie.year,
                                        DiscoveryRatingOption::UpVote
                                    ),
                                    &on_click(
                                        movie.name.to_string(),
                                        movie.year,
                                        DiscoveryRatingOption::DownVote
                                    ),
                                    &on_click(
                                        movie.name.to_string(),
                                        movie.year,
                                        DiscoveryRatingOption::Skip
                                    ),
                                )
                            })
                        }).collect::<Html>()
                    }
                </div>
            </section>
      </>
    }
}
