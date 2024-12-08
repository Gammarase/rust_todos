use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::components::{login::Login, todos::Todos, registration::Registration};

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Login{},
    #[route("/todos")]
    Todos{},
    #[route("/registration")]
    Registration{},
}
