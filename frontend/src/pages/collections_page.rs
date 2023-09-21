use crate::api::collections_api::api_get_user_collections;
use crate::components::header::Header;
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
    match media {
        Media::Movie(movie) => {
            //let movie_entry = format!("{} - {}", movie.name, movie.year);
            html! {<li>{format!("{} - {}", movie.name, movie.year)}</li>}
        }
        Media::TvShow(tv_show) => {
            html! {<li></li>}
        }
    }
}

#[function_component(CollectionsPage)]
pub fn collections_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let navigator = use_navigator().unwrap();
    let user = store.auth_user.clone();
    if user.is_none() {
        navigator.push(&router::Route::LoginPage);
    }
    let mut collections = use_state(|| store.collections.clone().unwrap_or_else(|| vec![]));

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
    let active_collection: &[UserCollection] = &collections[..];
    html! {
      <>
        <Header />
        <section class="flex flex-row items-stretch justify-center">
            if active_collection.is_empty() {
                <p>{"You have no collections! You should create one!"}</p>
            } else {
                {
                    active_collection.into_iter().map(|col| {
                        html!{
                            <div class="card">
                                <div class="card-body">
                                <h2 class="card-title">{ format!("{}",col.name) }</h2>
                                <ul>
                                    {
                                        col.collection.entries.iter().map(|media| {
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
        </section>
      </>
    }
}
