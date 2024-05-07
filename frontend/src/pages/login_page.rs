use crate::api::user_api::{api_login_user, api_user_info};
use crate::components::{form_input::FormInput, loading_button::LoadingButton};
use crate::router::{self, Route};
use crate::store::{set_auth_user, set_page_loading, set_show_alert, Store};
use gloo::console::console;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
struct LoginUserSchema {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    email: String,
    #[validate(
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    password: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<LoginUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "email" => data.email = value,
            "password" => data.password = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let form = use_state(LoginUserSchema::default);
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator().unwrap();

    let email_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();

    // let validate_input_on_blur = {
    //     let cloned_form = form.clone();
    //     let cloned_validation_errors = validation_errors.clone();
    //     Callback::from(move |(name, value): (String, String)| {
    //         console!(format!("{name:?} -- {value:?}"));
    //         let mut data = cloned_form.deref().clone();
    //         match name.as_str() {
    //             "email" => data.email = value,
    //             "password" => data.password = value,
    //             _ => (),
    //         }
    //         console!(format!("{cloned_form:?}"));
    //         cloned_form.set(data);
    //
    //         match cloned_form.validate() {
    //             Ok(_) => {
    //                 cloned_validation_errors
    //                     .borrow_mut()
    //                     .errors_mut()
    //                     .remove(name.as_str());
    //             }
    //             Err(errors) => {
    //                 cloned_validation_errors
    //                     .borrow_mut()
    //                     .errors_mut()
    //                     .retain(|key, _| key != &name);
    //                 for (field_name, error) in errors.errors() {
    //                     if field_name == &name {
    //                         cloned_validation_errors
    //                             .borrow_mut()
    //                             .errors_mut()
    //                             .insert(field_name.clone(), error.clone());
    //                     }
    //                 }
    //             }
    //         }
    //     })
    // };

    let handle_email_input = get_input_callback("email", form.clone());
    let handle_password_input = get_input_callback("password", form.clone());

    let on_submit = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();

        let store_dispatch = dispatch.clone();
        let cloned_navigator = navigator.clone();

        let cloned_email_input_ref = email_input_ref.clone();
        let cloned_password_input_ref = password_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let dispatch = store_dispatch.clone();
            let form = cloned_form.clone();
            let validation_errors = cloned_validation_errors.clone();
            let navigator = cloned_navigator.clone();

            let email_input_ref = cloned_email_input_ref.clone();
            let password_input_ref = cloned_password_input_ref.clone();

            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        let form_data = form.deref().clone();
                        set_page_loading(true, &dispatch);

                        let email_input = email_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();

                        email_input.set_value("");
                        password_input.set_value("");

                        let form_json = serde_json::to_string(&form_data).unwrap();
                        let res = api_login_user(&form_json).await;
                        match res {
                            Ok(_) => {
                                set_page_loading(false, &dispatch);
                                match api_user_info().await {
                                    Ok(user) => {
                                        set_auth_user(Some(user), &dispatch);
                                    }
                                    Err(e) => {
                                        console!(format!("{:?}", e));
                                    }
                                }

                                navigator.push(&router::Route::CollectionsPage);
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

    html! {
        <>
        // <Header />
        <section class="grid place-items-center">
            <h1 class="text-4xl xl:text-6xl text-center font-[600] mt-4 mb-4">{"Login"}</h1>
            <div class="card w-96 shadow-xl bg-base-200">
                <form
                        onsubmit={on_submit}
                        class="max-w-md w-full mx-auto overflow-hidden shadow-lg p-8 space-y-5"
                >
                    <FormInput
                            input_type="email"
                            label="Email"
                            name="email"
                            placeholder="your@login.email"
                            input_ref={email_input_ref}
                            handle_onchange={handle_email_input}
                            errors={&*validation_errors}
                            handle_on_input_blur={Callback::noop()}
                    />
                    <FormInput
                            input_type="password"
                            label="Password"
                            name="password"
                            placeholder="Hunter123"
                            input_ref={password_input_ref}
                            handle_onchange={handle_password_input}
                            errors={&*validation_errors}
                            handle_on_input_blur={Callback::noop()}
                    />
                    <div class="m-0">
                        <button class="btn btn-block" type="submit">{"Login"}</button>
                        <br />
                        <button class="btn btn-ghost btn-block">
                            <Link<Route> to={Route::RegisterPage} classes="text-ct-dark-600">
                                {"Sign Up"}
                            </Link<Route>>
                        </button>
                    </div>
                    // TODO: Actually implement password reset...
                    <div class="divider divider-warning pb-4">{"!"}</div>
                    <a class="pt-4" href="#">
                        {"//TODO: Forgot Password?"}
                    </a>
                </form>
            </div>
        </section>
        </>
    }
}
