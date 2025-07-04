use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::features::home::home_view::Home;
use crate::features::not_found::not_found_view::NotFound;
use crate::features::profile::profile_view::Profile;

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <NotFound/> }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/profile") view=Profile />
            </Routes>
        </Router>
    }
}