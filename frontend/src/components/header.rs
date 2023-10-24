use crate::components::figures;
use crate::{
    api::user_api::api_logout_user,
    router::{self, Route},
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
};
use figures::RoboHead;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::ui_helpers::redirect_to;

#[function_component(Header)]
pub fn header_component() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();

    let handle_logout = {
        let store_dispatch = dispatch.clone();
        let cloned_navigator = navigator.clone();

        Callback::from(move |_| {
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

    let handle_farts = {
        Callback::from(move |_: MouseEvent| {
            redirect_to("/login");
        })
    };

    html! {
        <div class="navbar bg-base-100 lg:justify-center">
            if user.is_some() { // Show links.
                <div class="justify-center">
                    <div class="">
                        <div class="btn" onclick={handle_farts}>{"FARTS!"}</div>
                    </div>
                    <div class="divider divider-horizontal"></div>
                    <div class="">
                        <Link<Route> to={Route::HomePage} classes="text-ct-dark-600">
                            {"Home"}
                        </Link<Route>>
                    </div>
                    <div class="divider divider-horizontal"></div>
                    <div class="">
                        <Link<Route> to={Route::DiscoveryPage} classes="text-ct-dark-600">{"Discover"}</Link<Route>>
                    </div>
                    <div class="divider divider-horizontal"></div>
                    <div class="">
                        <div class="dropdown dropdown-end">
                            <label tabindex="0" class="btn btn-ghost btn-square">
                                {"BA!ng"}
                                // <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                                // A cute little robot head :)
                                <figure>
                                    <RoboHead />
                                </figure>
                            </label>
                            <ul tabindex="0" class="mt-3 z-[1] p-2 shadow menu menu-sm dropdown-content bg-base-200 rounded-box w-32">
                                <li>
                                    <Link<Route> to={Route::ProfilePage} classes="text-ct-dark-600">
                                        {"Profile"}
                                    </Link<Route>>
                                </li>
                                <li onclick={&handle_logout}>
                                    <a>{"Logout"}</a>
                                </li>
                            </ul>
                        </div>
                    </div>
                    <div class="divider divider-horizontal"></div>
                    <div class="">
                        <Link<Route> to={Route::CollectionsPage} classes="text-ct-dark-600">{"Collections"}</Link<Route>>
                    </div>
                    <div class="divider divider-horizontal"></div>
                    <div class="">
                        <Link<Route> to={Route::SchedulesPage} classes="text-ct-dark-600">{"Schedules"}</Link<Route>>
                    </div>
                </div>
            } else {
                <div class="">
                    <div class="dropdown dropdown-end">
                        <label tabindex="0" class="btn btn-ghost btn-square">
                            {"BA!ng"}
                            // <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                            // A cute little robot head :)
                            <figure>
                                <RoboHead />
                            </figure>
                        </label>
                        <ul tabindex="0" class="mt-3 z-[1] p-2 shadow menu menu-sm dropdown-content bg-base-200 rounded-box w-32">

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

                        </ul>
                    </div>
                </div>
            }
        </div>
    }
}
