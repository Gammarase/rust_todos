#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::launch;
use dioxus_router::prelude::*;
mod router {
    pub mod route;
}
mod components {
    pub mod todos;
    pub mod login;
    pub mod registration;
    pub mod auth_state;
}
mod datasources{
    pub mod datarepository;
}

pub fn App(cx: Scope) -> Element {
    let auth_state = components::auth_state::AuthState::default();
    use_shared_state_provider(cx,|| auth_state.clone());
    render! {
        Router::<router::route::Route> {}
    }
}

fn main() {
    launch(App);
}
