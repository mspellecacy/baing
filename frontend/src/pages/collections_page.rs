use std::future::Future;
use gloo::console::console;
use crate::components::header::Header;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::prelude::Redirect;
use yewdux::functional::use_store;
use crate::api::collections_api::api_get_user_collections;
use crate::router;
use crate::store::{set_page_loading, set_show_alert, Store};

#[derive(Clone, Debug, PartialEq)]
struct ContentCollections {
    collections: Vec<String>,
}

#[function_component(CollectionsPage)]
pub fn collections_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let navigator = use_navigator().unwrap();
    let user = store.auth_user.clone();
    if user.is_none() {
        navigator.push(&router::Route::LoginPage);
    }
    let collections = store.collections.clone().unwrap_or_else(|| vec![]);

    {
        use_effect_with_deps(
            move |_| {
                let dispatch = dispatch.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    set_page_loading(true, &dispatch);
                    let response = api_get_user_collections().await;
                    match response {
                        Ok(collections) => {
                            set_page_loading(false, &dispatch);

                        }
                        Err(e) => {
                            console!(e.clone());
                            set_page_loading(false, &dispatch);
                            if e.contains("You are not logged in") {
                                navigator.push(&router::Route::LoginPage);
                            }
                            set_show_alert(e.to_string(), &dispatch);
                        }
                    }
                });
                || ()
            }, ()
        );
    }

    html! {
      <>
        <Header />
        <section class="bg-ct-blue-600 min-h-screen pt-20">
            <div class="max-w-4xl mx-auto bg-ct-dark-100 rounded-md h-[20rem] flex justify-center items-center">
                <p class="text-3xl font-semibold">{"Your Media Collections Go Here!"}</p>
                if collections.is_empty() {
                    <p>{"You have no collections! You should create one!"}</p>
                } else {
                    <p>{format!("{collections:?}")}</p>
                }
            </div>
        </section>
      </>
    }
}