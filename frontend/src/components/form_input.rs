use std::ops::Not;
use std::{cell::RefCell, rc::Rc};

use validator::ValidationErrors;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Option<String>,
    pub input_type: Option<String>,
    pub label: String,
    pub name: String,
    pub placeholder: Option<String>,
    pub value: Option<String>, // Everything is a String! WAHOO!
    pub input_ref: NodeRef,
    pub handle_onchange: Callback<String>,
    pub handle_on_input_blur: Callback<(String, String)>,
    pub errors: Rc<RefCell<ValidationErrors>>,
    pub disabled: Option<bool>,
}

#[function_component(FormInput)]
pub fn form_input_component(props: &Props) -> Html {
    let input_type = props
        .input_type
        .clone()
        .unwrap_or_else(|| "text".to_string());
    let val_errors = props.errors.borrow();
    let errors = val_errors.field_errors().clone();
    let empty_errors = vec![];
    let error = match errors.get(&props.name.as_str()) {
        Some(error) => error,
        None => &empty_errors,
    };
    let error_message = match error.get(0) {
        Some(message) => message.to_string(),
        None => "".to_string(),
    };

    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        handle_onchange.emit(value);
    });

    let handle_on_input_blur = props.handle_on_input_blur.clone();
    let on_blur = {
        let cloned_input_name = props.name.clone();
        Callback::from(move |event: FocusEvent| {
            let input_name = cloned_input_name.clone();
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            handle_on_input_blur.emit((input_name, value));
        })
    };
    let input_error = error_message.is_empty().not().then_some("input-error");

    html! {
    <div class="form-control w-full max-w-xs">
      <label html={props.name.clone()} class="label">
        <span class="label-text">
            {props.label.clone()}
        </span>
      </label>
      <input
        id={props.id.clone()}
        type={input_type}
        placeholder={props.placeholder.clone().unwrap_or(String::new())}
        class={classes!(
            "input", "input-bordered", "w-full", "max-w-xs",
            &input_error
        )}
        ref={props.input_ref.clone()}
        onchange={onchange}
        onblur={on_blur}
        value={props.value.clone()}
        disabled={props.disabled.unwrap_or(false)}
      />
    <span class="label text-xs pt-1">
        <span class="label-text-alt text-error">
            {error_message}
        </span>
    </span>
    </div>
    }
}
