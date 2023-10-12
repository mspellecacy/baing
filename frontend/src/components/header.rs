use crate::{
    api::user_api::api_logout_user,
    router::{self, Route},
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Header)]
pub fn header_component() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();

    let handle_logout = {
        let store_dispatch = dispatch.clone();
        let cloned_navigator = navigator.clone();

        Callback::from(move |_: MouseEvent| {
            let dispatch = store_dispatch.clone();
            let navigator = cloned_navigator.clone();
            spawn_local(async move {
                set_page_loading(true, &dispatch);
                let res = api_logout_user().await;
                match res {
                    Ok(_) => {
                        set_page_loading(false, &dispatch);
                        set_auth_user(None, &dispatch);
                        set_show_alert("Logged out successfully".to_string(), &dispatch);
                        navigator.push(&router::Route::LoginPage);
                    }
                    Err(e) => {
                        set_show_alert(e.to_string(), &dispatch);
                        set_page_loading(false, &dispatch);
                    }
                };
            });
        })
    };

    html! {
        <div class="navbar bg-base-100 flex flex-row">
            <div class="flex-1">
                <Link<Route> to={Route::HomePage} classes="btn btn-ghost normal-case text-xl">
                    {"BA!ng"}
                </Link<Route>>
            </div>
            if user.is_some() { // Show links.
                <div class="flex-initial">
                    <Link<Route> to={Route::DiscoveryPage} classes="text-ct-dark-600">{"Discover"}</Link<Route>>
                </div>
                <div class="divider divider-horizontal"></div>
                <div class="flex">
                    <Link<Route> to={Route::CollectionsPage} classes="text-ct-dark-600">{"Collections"}</Link<Route>>
                </div>
                <div class="divider divider-horizontal"></div>
                <div class="flex-1">
                    <Link<Route> to={Route::SchedulesPage} classes="text-ct-dark-600">{"Schedules"}</Link<Route>>
                </div>
            }
            <div class="flex-none gap-2">
                <div class="dropdown dropdown-end">
                    <label tabindex="0" class="btn btn-ghost btn-square">
                        // <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                        // A cute little robot head :)
                        <figure>
                            <svg
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                        d="M7 14C8.10457 14 9 13.1046 9 12C9 10.8954 8.10457 10 7 10C5.89543 10 5 10.8954 5 12C5 13.1046 5.89543 14 7 14Z"
                                        fill="currentColor"
                                />
                                <path
                                        d="M19 12C19 13.1046 18.1046 14 17 14C15.8954 14 15 13.1046 15 12C15 10.8954 15.8954 10 17 10C18.1046 10 19 10.8954 19 12Z"
                                        fill="currentColor"
                                />
                                <path
                                        fill-rule="evenodd"
                                        clip-rule="evenodd"
                                        d="M7 5C3.13401 5 0 8.13401 0 12C0 15.866 3.13401 19 7 19H17C20.866 19 24 15.866 24 12C24 8.13401 20.866 5 17 5H7ZM17 8H7C4.79086 8 3 9.79086 3 12C3 14.2091 4.79086 16 7 16H17C19.2091 16 21 14.2091 21 12C21 9.79086 19.2091 8 17 8Z"
                                        fill="currentColor"
                                />
                            </svg>
                        </figure>
                    </label>
                    <ul tabindex="0" class="mt-3 z-[1] p-2 shadow menu menu-sm dropdown-content bg-base-100 rounded-box w-52">
                    if user.is_some() {
                        <>
                            <li>
                                <Link<Route> to={Route::ProfilePage} classes="text-ct-dark-600">
                                    {"Profile"}
                                </Link<Route>>
                            </li>
                            <li onclick={handle_logout}>
                                <a>{"Logout"}</a>
                            </li>
                        </>
                    } else {
                        <>
                            <li>
                                <Link<Route> to={Route::RegisterPage} classes="text-ct-dark-600">
                                    {"Sign Up"}
                                </Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::LoginPage} classes="text-ct-dark-600">
                                    {"Login"}
                                </Link<Route>>
                            </li>
                        </>
                    }
                    </ul>
                </div>
            </div>
        </div>
    }
}
