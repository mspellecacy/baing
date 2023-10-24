use crate::components::{
    alert::{AlertComponent, Props as AlertProps},
    header::Header,
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use crate::router::{switch, Route};
use crate::store::Store;

#[function_component(App)]
pub fn app() -> Html {
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    // let is_page_loading = store.page_loading;

    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
    };
    html! {
        <BrowserRouter>
                <div>
                    <Header />

                    <Switch<Route> render={switch} />

                    if show_alert {
                        <AlertComponent
                            message={alert_props.message}
                            delay_ms={alert_props.delay_ms}
                         />
                    }
                </div>
                // if is_page_loading {
                //     <section class="grid place-content-center items-start min-h-screen">
                //         <div>
                //             <Spinner />
                //         </div>
                //     </section>
                // }
        </BrowserRouter>
    }
}
