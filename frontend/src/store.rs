use common::model::collections::UserCollection;
use common::model::user::User;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct AlertInput {
    pub show_alert: bool,
    pub alert_message: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
pub struct Store {
    pub auth_user: Option<User>,
    pub page_loading: bool,
    pub alert_input: AlertInput,
    pub collections: Option<Vec<UserCollection>>,
    pub schedules: Option<Vec<String>>,
}

pub fn set_page_loading(loading: bool, dispatch: &Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.page_loading = loading;
    })
}

pub fn set_auth_user(user: Option<User>, dispatch: &Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.auth_user = user;
    })
}

pub fn set_show_alert(message: String, dispatch: &Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input = AlertInput {
            alert_message: message,
            show_alert: true,
        };
    })
}

pub fn set_hide_alert(dispatch: &Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input.show_alert = false;
    })
}
