use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::schedules_page::SchedulesPage;
use crate::pages::{
    collections_page::CollectionsPage, discovery_page::DiscoveryPage, home_page::HomePage,
    login_page::LoginPage, profile_page::ProfilePage, register_page::RegisterPage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/register")]
    RegisterPage,
    #[at("/login")]
    LoginPage,
    #[at("/profile")]
    ProfilePage,
    #[at("/collections")]
    CollectionsPage,
    #[at("/discovery")]
    DiscoveryPage,
    #[at("/schedules")]
    SchedulesPage,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage/> },
        Route::RegisterPage => html! {<RegisterPage/> },
        Route::LoginPage => html! {<LoginPage/> },
        Route::ProfilePage => html! {<ProfilePage/> },
        Route::CollectionsPage => html! {<CollectionsPage/> },
        Route::DiscoveryPage => html! {<DiscoveryPage/> },
        Route::SchedulesPage => html! {<SchedulesPage/> },
    }
}
