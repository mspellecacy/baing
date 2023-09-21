use crate::api::user_api::api_update_user;
use crate::components::{form_input::FormInput, loading_button::LoadingButton};
use crate::ui_helpers::UiHelpers;
use crate::{
    api::user_api::api_user_info,
    components::header::Header,
    router,
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
};
use common::model::user::UserUpdateData;
use gloo::console::console;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
struct UpdateUserSchema {
    id: String,
    #[validate(length(min = 3, message = "Display Name is required"))]
    name: String,
    email: String,
    role: String,
    photo: String,
    verified: bool,
    tmdb_api_key: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<UpdateUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "name" => data.name = value,
            "tmdb_api_key" => data.tmdb_api_key = value,
            _ => (),
        };
        cloned_form.set(data);
    })
}

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let form = use_state(|| UpdateUserSchema::default());
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator().unwrap();

    let name_input_ref = NodeRef::default();
    let handle_name_input = get_input_callback("name", form.clone());
    let apikey_input_ref = NodeRef::default();
    let handle_apikey_input = get_input_callback("tmdb_api_key", form.clone());

    let on_submit = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        let cloned_navigator = navigator.clone();
        let cloned_dispatch = dispatch.clone();

        let cloned_name_input_ref = name_input_ref.clone();
        let cloned_apikey_input_ref = apikey_input_ref.clone();
        Callback::from(move |event: SubmitEvent| {
            let form = cloned_form.clone();
            let validation_errors = cloned_validation_errors.clone();
            let navigator = cloned_navigator.clone();
            let dispatch = cloned_dispatch.clone();

            let name_input_ref = cloned_name_input_ref.clone();
            let apikey_input_ref = cloned_apikey_input_ref.clone();

            event.prevent_default();
            spawn_local(async move {
                match form.validate() {
                    Ok(v) => {
                        let form_data = form.deref().clone();
                        let form_json = serde_json::to_string(&form_data).unwrap();
                        let user_update = UserUpdateData {
                            name: form_data.name.clone(),
                            tmdb_api_key: form_data.tmdb_api_key.clone(),
                        };
                        set_page_loading(true, &dispatch);

                        let name_input = name_input_ref.cast::<HtmlInputElement>().unwrap();
                        let apikey_input = apikey_input_ref.cast::<HtmlInputElement>().unwrap();

                        let update_response = api_update_user(user_update).await;
                        match update_response {
                            Ok(usr) => {
                                console!(format!("Profile Updated - {:?}", usr));
                                set_page_loading(false, &dispatch);
                                if let Ok(user) = api_user_info().await {
                                    set_auth_user(Some(user), &dispatch);
                                    set_show_alert("Profile Updated".to_string(), &dispatch);
                                } else {
                                    set_show_alert("Failed Profile Updated".to_string(), &dispatch);
                                }
                            }
                            Err(e) => {
                                set_page_loading(false, &dispatch);
                                set_show_alert(e.to_string(), &dispatch);
                            }
                        };
                    }
                    Err(e) => {
                        validation_errors.set(Rc::new(RefCell::new(e)));
                    }
                }
            });
        })
    };

    // Fetch User's profile from backend API.
    {
        let form = form.clone();
        use_effect_with_deps(
            move |_| {
                let dispatch = dispatch.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    set_page_loading(true, &dispatch);
                    let response = api_user_info().await;
                    match response {
                        Ok(user) => {
                            set_page_loading(false, &dispatch);
                            //set_auth_user(Some(user.clone()), dispatch);
                            form.set(UpdateUserSchema {
                                id: user.id.to_string(),
                                email: user.email.to_owned(),
                                photo: user.photo.to_owned(),
                                role: user.role.to_owned(),
                                name: user.name.clone(),
                                verified: user.verified,
                                tmdb_api_key: user.tmdb_api_key.unwrap_or_else(|| String::new()),
                            });
                        }
                        Err(e) => {
                            set_page_loading(false, &dispatch);
                            if e.contains("You are not logged in") {
                                navigator.push(&router::Route::LoginPage);
                            }
                            set_show_alert(e.to_string(), &dispatch);
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
    <>
      <Header />
      <div class="pt-6">
      if let Some(ref user) = user {
        <section class="grid place-items-center">
            <div class="card bg-base-200 w-100 shadow-xl text-neutral-content">
                <div class="card-body">
                    <h1 class="mb-4 card-title">
                        {"Profile"}
                    </h1>
                    <form onsubmit={on_submit}>
                        <FormInput
                            id="bng_UserDisplayName"
                            label="Display Name"
                            name="name"
                            placeholder="Display Name"
                            input_ref={name_input_ref}
                            handle_onchange={handle_name_input}
                            errors={&*validation_errors}
                            handle_on_input_blur={Callback::noop()}
                            value={(*form.name).to_string()}
                        />
                        <FormInput
                            id="bng_UserTmdbApiKey"
                            label="TMDB API Key"
                            name="tmdb_api_key"
                            placeholder="TMDB API Key"
                            input_ref={apikey_input_ref}
                            handle_onchange={handle_apikey_input}
                            errors={&*validation_errors}
                            handle_on_input_blur={Callback::noop()}
                            value={(*form.tmdb_api_key).to_string()}
                        />
                        <button class="btn btn-wide btn-secondary btn-outline">{"Save"}</button>
                        <div class="divider"></div>
                        <FormInput
                            id="bng_UserId"
                            label="User ID"
                            name="user_id"
                            placeholder="User ID"
                            input_ref={NodeRef::default()}
                            handle_onchange={Callback::noop()}
                            errors={&*validation_errors}
                            handle_on_input_blur={Callback::noop()}
                            value={(*form.id).to_string()}
                            disabled={true}
                        />
                        <FormInput
                            id="bng_UserEmail"
                            label="Email"
                            name="email"
                            placeholder="Email"
                            input_ref={NodeRef::default()}
                            handle_onchange={Callback::noop()}
                            errors={&*validation_errors}
                            handle_on_input_blur={Callback::noop()}
                            value={(*form.email).to_string()}
                            disabled={true}
                        />
                        <FormInput
                            id="bng_UserVerified"
                            label="Verified"
                            name="verified"
                            placeholder="Verified"
                            input_ref={NodeRef::default()}
                            handle_onchange={Callback::noop()}
                            errors={&*validation_errors}
                            handle_on_input_blur={Callback::noop()}
                            value={(form.verified).to_string()}
                            disabled={true}
                        />
                        <FormInput
                            id="bng_UserRole"
                            label="Role"
                            name="role"
                            placeholder="Role"
                            input_ref={NodeRef::default()}
                            handle_onchange={Callback::noop()}
                            errors={&*validation_errors}
                            handle_on_input_blur={Callback::noop()}
                            value={(*form.role).to_string()}
                            disabled={true}
                        />
                        // <FormInput
                        //     id="bng_UserPhoto"
                        //     label="Photo"
                        //     name="photo"
                        //     placeholder="Photo"
                        //     input_ref={NodeRef::default()}
                        //     handle_onchange={Callback::noop()}
                        //     errors={&*validation_errors}
                        //     handle_on_input_blur={Callback::noop()}
                        //     value={(*form.photo).to_string()}
                        //     disabled={true}
                        // />
                    </form>
                </div>
            </div>
        </section>
    } else {
        <p class="mb-4">{"Loading..."}</p>
    }
        </div>
    </>
    }
}
